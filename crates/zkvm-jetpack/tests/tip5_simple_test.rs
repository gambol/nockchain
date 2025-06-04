use std::time::Instant;
use zkvm_jetpack::form::math::tip5::*;

#[test]
fn test_tip5_basic_functionality() {
    println!("🔥 TIP5 Basic Functionality Test");
    
    // Test the basic TIP5 permutation function
    let mut test_sponge = [0u64; STATE_SIZE];
    for i in 0..STATE_SIZE {
        test_sponge[i] = 0x1234567890abcdef + (i as u64);
    }
    
    let original_sponge = test_sponge;
    permute(&mut test_sponge);
    
    // Verify that the permutation actually changed the values
    let mut changed = false;
    for i in 0..STATE_SIZE {
        if test_sponge[i] != original_sponge[i] {
            changed = true;
            break;
        }
    }
    
    assert!(changed, "TIP5 permutation should change the input values");
    println!("✅ TIP5 permutation function works correctly");
}

#[test]
fn test_tip5_performance() {
    println!("📈 TIP5 Performance Test");
    
    let mut test_sponge = [0u64; STATE_SIZE];
    for i in 0..STATE_SIZE {
        test_sponge[i] = 0x1234567890abcdef + (i as u64);
    }
    
    // Warm up
    for _ in 0..100 {
        let mut sponge = test_sponge;
        permute(&mut sponge);
    }
    
    // Performance test
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut sponge = test_sponge;
        permute(&mut sponge);
    }
    
    let duration = start.elapsed();
    let avg_time = duration / iterations;
    let ops_per_sec = 1.0 / avg_time.as_secs_f64();
    
    println!("📊 Performance Results:");
    println!("   Iterations: {}", iterations);
    println!("   Total time: {:?}", duration);
    println!("   Average time per permutation: {:?}", avg_time);
    println!("   Permutations per second: {:.0}", ops_per_sec);
    
    // Performance assertion - should be reasonably fast
    assert!(avg_time.as_millis() < 10, "Each permutation should take less than 10ms");
    
    if avg_time.as_micros() < 10 {
        println!("🚀 Performance: EXCELLENT (< 10μs per permutation)");
    } else if avg_time.as_micros() < 100 {
        println!("✅ Performance: GOOD (< 100μs per permutation)");
    } else if avg_time.as_micros() < 1000 {
        println!("⚠️  Performance: ACCEPTABLE (< 1ms per permutation)");
    } else {
        println!("❌ Performance: SLOW (> 1ms per permutation)");
    }
}

#[test]
fn test_tip5_components() {
    println!("🔧 TIP5 Components Test");
    
    let test_state = [0x1234567890abcdef; STATE_SIZE];
    
    // Test sbox_layer
    let sbox_result = sbox_layer(&test_state);
    assert_ne!(sbox_result, test_state, "S-box layer should change the input");
    
    // Test linear_layer
    let linear_result = linear_layer(&test_state);
    assert_ne!(linear_result, test_state, "Linear layer should change the input");
    
    println!("✅ All TIP5 components work correctly");
}

#[test]
fn test_tip5_deterministic() {
    println!("🎯 TIP5 Deterministic Test");
    
    let test_input = [0x1111111111111111; STATE_SIZE];
    
    // Run the same permutation multiple times
    let mut results = Vec::new();
    for _ in 0..5 {
        let mut sponge = test_input;
        permute(&mut sponge);
        results.push(sponge);
    }
    
    // All results should be identical
    for i in 1..results.len() {
        assert_eq!(results[0], results[i], "TIP5 permutation should be deterministic");
    }
    
    println!("✅ TIP5 permutation is deterministic");
}

#[test]
fn test_tip5_different_inputs() {
    println!("🔄 TIP5 Different Inputs Test");
    
    let inputs = vec![
        [0; STATE_SIZE],
        [1; STATE_SIZE],
        [0xffffffffffffffff; STATE_SIZE],
        {
            let mut input = [0; STATE_SIZE];
            for i in 0..STATE_SIZE {
                input[i] = i as u64;
            }
            input
        },
    ];
    
    let mut outputs = Vec::new();
    
    for input in inputs.iter() {
        let mut sponge = *input;
        permute(&mut sponge);
        outputs.push(sponge);
    }
    
    // All outputs should be different
    for i in 0..outputs.len() {
        for j in i+1..outputs.len() {
            assert_ne!(outputs[i], outputs[j], "Different inputs should produce different outputs");
        }
    }
    
    println!("✅ TIP5 produces different outputs for different inputs");
}
