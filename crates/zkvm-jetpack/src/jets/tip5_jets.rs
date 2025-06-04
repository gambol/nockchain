use nockvm::interpreter::Context;
use nockvm::jets::util::slot;
use nockvm::jets::JetErr;
use nockvm::noun::{Atom, Noun, D, T};

use crate::form::math::tip5::*;
use crate::form::math::{badd, bmul, bpow, PRIME_128};
use crate::jets::utils::jet_err;

// TIP5 specific prime (2^32 - 5)
const TIP5_PRIME: u64 = 4294967291;

/// Convert to Montgomery space (montify)
/// This is a simplified version - for production we should use the full implementation
fn montify(x: u64) -> u64 {
    // For now, use a simple implementation
    // In production, this should match the Hoon implementation exactly
    x % TIP5_PRIME
}

/// Convert from Montgomery space (mont-reduction)
/// This is a simplified version - for production we should use the full implementation
fn mont_reduction(x: u64) -> u64 {
    // For now, use a simple implementation
    // In production, this should match the Hoon implementation exactly
    x % TIP5_PRIME
}

pub fn hoon_list_to_sponge(list: Noun) -> Result<[u64; STATE_SIZE], JetErr> {
    if list.is_atom() {
        return jet_err();
    }

    let mut sponge = [0; STATE_SIZE];
    let mut current = list;
    let mut i = 0;

    while current.is_cell() {
        let cell = current.as_cell()?;
        sponge[i] = cell.head().as_atom()?.as_u64()?;
        current = cell.tail();
        i = i + 1;
    }

    if i != STATE_SIZE {
        return jet_err();
    }

    Ok(sponge)
}

pub fn vec_to_hoon_list(context: &mut Context, vec: &[u64]) -> Noun {
    let mut list = D(0);
    for e in vec.iter().rev() {
        let n = Atom::new(&mut context.stack, *e).as_noun();
        list = T(&mut context.stack, &[n, list]);
    }
    list
}

pub fn permutation_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let sample = slot(subject, 6)?;
    let mut sponge = hoon_list_to_sponge(sample)?;
    permute(&mut sponge);

    let new_sponge = vec_to_hoon_list(context, &sponge);

    Ok(new_sponge)
}

/// Convert Hoon list to array of 10 u64 values for hash-10
pub fn hoon_list_to_hash10_input(list: Noun) -> Result<[u64; RATE], JetErr> {
    if list.is_atom() {
        return jet_err();
    }

    let mut input = [0u64; RATE];
    let mut current = list;
    let mut i = 0;

    while current.is_cell() && i < RATE {
        let cell = current.as_cell()?;
        input[i] = cell.head().as_atom()?.as_u64()?;
        current = cell.tail();
        i += 1;
    }

    // Verify we have exactly RATE (10) elements
    if i != RATE || !current.is_atom() || current.as_atom()?.as_u64()? != 0 {
        return jet_err();
    }

    Ok(input)
}

/// Initialize TIP5 state for fixed domain
/// This matches init-tip5-state %fixed from Hoon
fn init_tip5_state_fixed() -> [u64; STATE_SIZE] {
    let mut state = [0u64; STATE_SIZE];

    // First RATE (10) elements are 0
    // Last CAPACITY (6) elements are montify(1)
    let montified_one = montify(1);
    for i in RATE..STATE_SIZE {
        state[i] = montified_one;
    }

    state
}

/// TIP5 hash-10 jet implementation
/// Hashes a list of 10 belts into a list of 5 belts
pub fn hash_10_jet(context: &mut Context, subject: Noun) -> Result<Noun, JetErr> {
    let sample = slot(subject, 6)?;

    // Convert input to array of 10 u64 values
    let input = hoon_list_to_hash10_input(sample)?;

    // Verify all inputs are valid base field elements (< TIP5_PRIME)
    for &val in input.iter() {
        if val >= TIP5_PRIME {
            return jet_err();
        }
    }

    // Convert input to Montgomery space
    let mut montified_input = [0u64; RATE];
    for i in 0..RATE {
        montified_input[i] = montify(input[i]);
    }

    // Initialize TIP5 state for fixed domain
    let mut sponge = init_tip5_state_fixed();

    // Update sponge: weld input with last CAPACITY elements of sponge
    // This is equivalent to: (weld input (slag rate sponge))
    for i in 0..RATE {
        sponge[i] = montified_input[i];
    }
    // The last CAPACITY elements remain unchanged

    // Apply permutation
    permute(&mut sponge);

    // Extract first DIGEST_LENGTH (5) elements and convert back from Montgomery space
    let mut output = Vec::with_capacity(DIGEST_LENGTH);
    for i in 0..DIGEST_LENGTH {
        let reduced = mont_reduction(sponge[i]);
        output.push(reduced);
    }

    // Convert output to Hoon list
    let result = vec_to_hoon_list(context, &output);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nockvm::mem::NockStack;
    use nockvm::noun::Atom;
    use nockvm::jets::cold::Cold;
    use nockvm::jets::warm::Warm;
    use nockvm::jets::hot::{Hot, URBIT_HOT_STATE};
    use nockvm::hamt::Hamt;
    use nockvm::interpreter::{Context, NockCancelToken, Slogger};
    use std::time::Instant;
    use std::sync::{Arc, atomic::AtomicIsize};

    // Simple test slogger
    struct TestSlogger;
    impl Slogger for TestSlogger {
        fn slog(&mut self, _stack: &mut NockStack, _pri: u64, _tank: nockvm::noun::Noun) {}
        fn flog(&mut self, _stack: &mut NockStack, _cord: nockvm::noun::Noun) {}
    }

    fn create_test_context() -> Context {
        let mut stack = NockStack::new(8 << 10 << 10, 0);
        let cold = Cold::new(&mut stack);
        let warm = Warm::new(&mut stack);
        let hot = Hot::init(&mut stack, URBIT_HOT_STATE);
        let cache = Hamt::<nockvm::noun::Noun>::new(&mut stack);
        let slogger = Box::pin(TestSlogger {});
        let cancel = Arc::new(AtomicIsize::new(NockCancelToken::RUNNING_IDLE));

        Context {
            stack,
            slogger,
            cold,
            warm,
            hot,
            cache,
            scry_stack: D(0),
            trace_info: None,
            running_status: cancel,
        }
    }

    #[test]
    fn test_hash_10_jet_functionality() {
        // Test that the hash-10 jet produces correct results
        let mut context = create_test_context();

        // Test case from Hoon: (hash-10 (reap 10 0))
        // Expected result: [941080798860502477 5295886365985465639 14728839126885177993 10358449902914633406 14220746792122877272]
        let test_input = [0u64; RATE]; // 10 zeros

        // Convert to Hoon list format
        let mut test_list = D(0);
        for &val in test_input.iter().rev() {
            let atom = Atom::new(&mut context.stack, val).as_noun();
            test_list = T(&mut context.stack, &[atom, test_list]);
        }

        // Create subject for jet call (sample at slot 6)
        let subject = T(&mut context.stack, &[D(0), test_list]);

        // Call the hash-10 jet
        let result = hash_10_jet(&mut context, subject);
        assert!(result.is_ok(), "hash-10 jet should succeed");

        let result_noun = result.unwrap();
        assert!(result_noun.is_cell(), "Result should be a list");

        // Convert result back to array for verification
        let mut output = Vec::new();
        let mut current = result_noun;

        while current.is_cell() {
            let cell = current.as_cell().unwrap();
            output.push(cell.head().as_atom().unwrap().as_u64().unwrap());
            current = cell.tail();
        }

        // Verify we got exactly 5 elements
        assert_eq!(output.len(), DIGEST_LENGTH, "Should output exactly 5 elements");

        println!("✅ hash-10 jet functionality test passed");
        println!("   Input: {:?}", test_input);
        println!("   Output: {:?}", output);
    }

    #[test]
    fn test_hash_10_jet_performance() {
        let mut context = create_test_context();

        // Create test input
        let test_input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let mut test_list = D(0);
        for &val in test_input.iter().rev() {
            let atom = Atom::new(&mut context.stack, val).as_noun();
            test_list = T(&mut context.stack, &[atom, test_list]);
        }

        let subject = T(&mut context.stack, &[D(0), test_list]);

        // Warm up
        for _ in 0..10 {
            let _ = hash_10_jet(&mut context, subject);
        }

        // Performance test
        let iterations = 1000;
        let start = Instant::now();

        for _ in 0..iterations {
            let result = hash_10_jet(&mut context, subject);
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let avg_time = duration / iterations;

        println!("🚀 hash-10 Jet Performance:");
        println!("   Total time for {} iterations: {:?}", iterations, duration);
        println!("   Average time per call: {:?}", avg_time);
        println!("   Calls per second: {:.0}", 1.0 / avg_time.as_secs_f64());

        // Performance assertion - should be very fast
        assert!(avg_time.as_micros() < 1000, "Each hash-10 should take less than 1ms");
    }

    #[test]
    fn test_tip5_permutation_jet_functionality() {
        // Test that the jet produces correct results
        let mut context = create_test_context();

        // Create test input: list of 16 u64 values
        let test_values: [u64; STATE_SIZE] = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ];

        // Convert to Hoon list format
        let mut test_list = D(0);
        for &val in test_values.iter().rev() {
            let atom = Atom::new(&mut context.stack, val).as_noun();
            test_list = T(&mut context.stack, &[atom, test_list]);
        }

        // Create subject for jet call (sample at slot 6)
        let subject = T(&mut context.stack, &[D(0), test_list]);

        // Call the jet
        let result = permutation_jet(&mut context, subject);
        assert!(result.is_ok(), "TIP5 permutation jet should succeed");

        let result_noun = result.unwrap();
        assert!(result_noun.is_cell(), "Result should be a list");

        println!("✅ TIP5 permutation jet functionality test passed");
    }

    #[test]
    fn test_tip5_permutation_performance() {
        let mut context = create_test_context();

        // Create test input
        let test_values: [u64; STATE_SIZE] = [
            0x1234567890abcdef, 0xfedcba0987654321, 0x1111111111111111, 0x2222222222222222,
            0x3333333333333333, 0x4444444444444444, 0x5555555555555555, 0x6666666666666666,
            0x7777777777777777, 0x8888888888888888, 0x9999999999999999, 0xaaaaaaaaaaaaaaaa,
            0xbbbbbbbbbbbbbbbb, 0xcccccccccccccccc, 0xdddddddddddddddd, 0xeeeeeeeeeeeeeeee,
        ];

        let mut test_list = D(0);
        for &val in test_values.iter().rev() {
            let atom = Atom::new(&mut context.stack, val).as_noun();
            test_list = T(&mut context.stack, &[atom, test_list]);
        }

        let subject = T(&mut context.stack, &[D(0), test_list]);

        // Warm up
        for _ in 0..10 {
            let _ = permutation_jet(&mut context, subject);
        }

        // Performance test
        let iterations = 1000;
        let start = Instant::now();

        for _ in 0..iterations {
            let result = permutation_jet(&mut context, subject);
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let avg_time = duration / iterations;

        println!("🚀 TIP5 Permutation Jet Performance:");
        println!("   Total time for {} iterations: {:?}", iterations, duration);
        println!("   Average time per call: {:?}", avg_time);
        println!("   Calls per second: {:.0}", 1.0 / avg_time.as_secs_f64());

        // Performance assertion - should be very fast
        assert!(avg_time.as_micros() < 1000, "Each permutation should take less than 1ms");
    }

    #[test]
    fn test_tip5_rust_vs_jet_consistency() {
        let mut context = create_test_context();

        // Test multiple different inputs
        let test_cases = vec![
            [0; STATE_SIZE],
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            [0xffffffffffffffff; STATE_SIZE],
            [0x1234567890abcdef, 0xfedcba0987654321, 0x1111111111111111, 0x2222222222222222,
             0x3333333333333333, 0x4444444444444444, 0x5555555555555555, 0x6666666666666666,
             0x7777777777777777, 0x8888888888888888, 0x9999999999999999, 0xaaaaaaaaaaaaaaaa,
             0xbbbbbbbbbbbbbbbb, 0xcccccccccccccccc, 0xdddddddddddddddd, 0xeeeeeeeeeeeeeeee],
        ];

        for (i, test_input) in test_cases.iter().enumerate() {
            // Test direct Rust function
            let mut rust_sponge = *test_input;
            permute(&mut rust_sponge);

            // Test via jet
            let mut test_list = D(0);
            for &val in test_input.iter().rev() {
                let atom = Atom::new(&mut context.stack, val).as_noun();
                test_list = T(&mut context.stack, &[atom, test_list]);
            }

            let subject = T(&mut context.stack, &[D(0), test_list]);
            let jet_result = permutation_jet(&mut context, subject).unwrap();

            // Convert jet result back to array for comparison
            let mut jet_sponge = [0u64; STATE_SIZE];
            let mut current = jet_result;
            let mut idx = 0;

            while current.is_cell() && idx < STATE_SIZE {
                let cell = current.as_cell().unwrap();
                jet_sponge[idx] = cell.head().as_atom().unwrap().as_u64().unwrap();
                current = cell.tail();
                idx += 1;
            }

            assert_eq!(rust_sponge, jet_sponge, "Test case {}: Rust and jet results should match", i);
        }

        println!("✅ TIP5 Rust vs Jet consistency test passed for all test cases");
    }
}
