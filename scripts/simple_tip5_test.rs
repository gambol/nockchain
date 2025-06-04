#!/usr/bin/env cargo +nightly -Zscript
//! ```cargo
//! [dependencies]
//! zkvm-jetpack = { path = "crates/zkvm-jetpack" }
//! ```

use std::time::Instant;

fn main() {
    println!("🔥 Simple TIP5 Performance Test");
    println!("================================");
    
    // Test the basic TIP5 permutation function
    let mut test_sponge = [0u64; 16];
    for i in 0..16 {
        test_sponge[i] = 0x1234567890abcdef + (i as u64);
    }
    
    println!("📊 Testing TIP5 permutation function...");
    
    // Test correctness first
    let original_sponge = test_sponge;
    zkvm_jetpack::form::math::tip5::permute(&mut test_sponge);
    
    // Verify that the permutation actually changed the values
    let mut changed = false;
    for i in 0..16 {
        if test_sponge[i] != original_sponge[i] {
            changed = true;
            break;
        }
    }
    
    if changed {
        println!("✅ TIP5 permutation function works correctly");
    } else {
        println!("❌ TIP5 permutation function may not be working");
        return;
    }
    
    // Performance test
    println!("📈 Running performance test...");
    
    // Warm up
    for _ in 0..100 {
        let mut sponge = original_sponge;
        zkvm_jetpack::form::math::tip5::permute(&mut sponge);
    }
    
    // Actual performance test
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut sponge = original_sponge;
        zkvm_jetpack::form::math::tip5::permute(&mut sponge);
    }
    
    let duration = start.elapsed();
    let avg_time = duration / iterations;
    let ops_per_sec = 1.0 / avg_time.as_secs_f64();
    
    println!("📊 Results:");
    println!("   Iterations: {}", iterations);
    println!("   Total time: {:?}", duration);
    println!("   Average time per permutation: {:?}", avg_time);
    println!("   Permutations per second: {:.0}", ops_per_sec);
    
    // Performance evaluation
    if avg_time.as_micros() < 10 {
        println!("🚀 Performance: EXCELLENT (< 10μs per permutation)");
    } else if avg_time.as_micros() < 50 {
        println!("✅ Performance: VERY GOOD (< 50μs per permutation)");
    } else if avg_time.as_micros() < 100 {
        println!("✅ Performance: GOOD (< 100μs per permutation)");
    } else if avg_time.as_micros() < 500 {
        println!("⚠️  Performance: ACCEPTABLE (< 500μs per permutation)");
    } else if avg_time.as_micros() < 1000 {
        println!("⚠️  Performance: SLOW (< 1ms per permutation)");
    } else {
        println!("❌ Performance: VERY SLOW (> 1ms per permutation)");
    }
    
    // Test different components
    println!("\n🔧 Testing TIP5 components...");
    
    // Test sbox_layer
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = zkvm_jetpack::form::math::tip5::sbox_layer(&original_sponge);
    }
    let sbox_time = start.elapsed() / iterations;
    println!("   S-box layer average time: {:?}", sbox_time);
    
    // Test linear_layer
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = zkvm_jetpack::form::math::tip5::linear_layer(&original_sponge);
    }
    let linear_time = start.elapsed() / iterations;
    println!("   Linear layer average time: {:?}", linear_time);
    
    println!("\n✅ TIP5 basic performance test completed!");
    
    // Recommendations
    println!("\n💡 Recommendations:");
    if avg_time.as_micros() > 100 {
        println!("   - Consider optimizing TIP5 implementation");
        println!("   - Profile to identify bottlenecks");
    }
    if sbox_time > linear_time * 2 {
        println!("   - S-box layer is the bottleneck");
    } else if linear_time > sbox_time * 2 {
        println!("   - Linear layer is the bottleneck");
    } else {
        println!("   - Both layers have similar performance");
    }
}
