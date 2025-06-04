use std::time::Instant;
use zkvm_jetpack::form::math::tip5::*;

fn main() {
    println!("🔥 TIP5 hash-10 vs Permutation Performance Comparison");
    println!("======================================================");
    
    // Test data for hash-10 (10 elements)
    let hash10_input = [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Test data for permutation (16 elements)
    let mut permutation_input = [0u64; STATE_SIZE];
    for i in 0..STATE_SIZE {
        permutation_input[i] = (i + 1) as u64;
    }
    
    println!("\n📊 Testing TIP5 permutation function...");
    
    // Warm up permutation
    for _ in 0..100 {
        let mut sponge = permutation_input;
        permute(&mut sponge);
    }
    
    // Test permutation performance
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut sponge = permutation_input;
        permute(&mut sponge);
    }
    
    let permutation_duration = start.elapsed();
    let permutation_avg = permutation_duration / iterations;
    let permutation_ops_per_sec = 1.0 / permutation_avg.as_secs_f64();
    
    println!("📈 TIP5 Permutation Results:");
    println!("   Iterations: {}", iterations);
    println!("   Total time: {:?}", permutation_duration);
    println!("   Average time per call: {:?}", permutation_avg);
    println!("   Operations per second: {:.0}", permutation_ops_per_sec);
    
    println!("\n📊 Testing hash-10 equivalent operations...");
    
    // Simulate hash-10 operations using direct permutation calls
    // This represents what hash-10 does internally
    
    // Warm up hash-10 simulation
    for _ in 0..100 {
        simulate_hash10(&hash10_input);
    }
    
    // Test hash-10 simulation performance
    let start = Instant::now();
    
    for _ in 0..iterations {
        simulate_hash10(&hash10_input);
    }
    
    let hash10_duration = start.elapsed();
    let hash10_avg = hash10_duration / iterations;
    let hash10_ops_per_sec = 1.0 / hash10_avg.as_secs_f64();
    
    println!("📈 hash-10 Simulation Results:");
    println!("   Iterations: {}", iterations);
    println!("   Total time: {:?}", hash10_duration);
    println!("   Average time per call: {:?}", hash10_avg);
    println!("   Operations per second: {:.0}", hash10_ops_per_sec);
    
    // Performance comparison
    println!("\n🏆 Performance Comparison:");
    println!("==========================");
    
    let speedup_ratio = permutation_avg.as_nanos() as f64 / hash10_avg.as_nanos() as f64;
    
    if hash10_avg < permutation_avg {
        println!("✅ hash-10 is {:.2}x FASTER than raw permutation", speedup_ratio);
    } else {
        println!("⚠️  hash-10 is {:.2}x SLOWER than raw permutation", 1.0 / speedup_ratio);
    }
    
    println!("   Raw permutation: {:?} per call", permutation_avg);
    println!("   hash-10 simulation: {:?} per call", hash10_avg);
    
    // Mining implications
    println!("\n🎯 Mining Performance Implications:");
    println!("===================================");
    
    let hash10_hashes_per_sec = hash10_ops_per_sec;
    println!("   hash-10 operations per second: {:.0}", hash10_hashes_per_sec);
    
    if hash10_hashes_per_sec > 1_000_000.0 {
        println!("   Mining hash rate potential: EXCELLENT (>1M hashes/sec)");
    } else if hash10_hashes_per_sec > 100_000.0 {
        println!("   Mining hash rate potential: GOOD (>100K hashes/sec)");
    } else if hash10_hashes_per_sec > 10_000.0 {
        println!("   Mining hash rate potential: ACCEPTABLE (>10K hashes/sec)");
    } else {
        println!("   Mining hash rate potential: NEEDS OPTIMIZATION (<10K hashes/sec)");
    }
    
    // Theoretical jet performance improvement
    println!("\n🚀 Expected Jet Performance Improvement:");
    println!("========================================");
    println!("   Current Rust implementation: {:.0} ops/sec", hash10_ops_per_sec);
    println!("   Expected with proper jet (3-5x): {:.0}-{:.0} ops/sec", 
             hash10_ops_per_sec * 3.0, hash10_ops_per_sec * 5.0);
    println!("   Expected vs Hoon interpretation (5-10x): {:.0}-{:.0} ops/sec", 
             hash10_ops_per_sec * 5.0, hash10_ops_per_sec * 10.0);
    
    println!("\n✅ Performance analysis completed!");
}

/// Simulate hash-10 operation using the same steps as the jet
fn simulate_hash10(input: &[u64; RATE]) -> [u64; DIGEST_LENGTH] {
    // Simulate Montgomery conversion (simplified)
    let mut montified_input = [0u64; RATE];
    for i in 0..RATE {
        montified_input[i] = input[i] % 4294967291; // TIP5_PRIME
    }
    
    // Initialize TIP5 state for fixed domain
    let mut sponge = [0u64; STATE_SIZE];
    
    // First RATE elements are input, last CAPACITY elements are montify(1)
    for i in 0..RATE {
        sponge[i] = montified_input[i];
    }
    let montified_one = 1u64 % 4294967291; // simplified montify(1)
    for i in RATE..STATE_SIZE {
        sponge[i] = montified_one;
    }
    
    // Apply permutation
    permute(&mut sponge);
    
    // Extract first DIGEST_LENGTH elements and convert back (simplified)
    let mut output = [0u64; DIGEST_LENGTH];
    for i in 0..DIGEST_LENGTH {
        output[i] = sponge[i] % 4294967291; // simplified mont_reduction
    }
    
    output
}
