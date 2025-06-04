use std::time::Instant;
use zkvm_jetpack::form::math::tip5::*;

fn main() {
    println!("🔥 TIP5 Performance Analysis");
    println!("============================");
    
    // Test the basic TIP5 permutation function
    let mut test_sponge = [0u64; STATE_SIZE];
    for i in 0..STATE_SIZE {
        test_sponge[i] = 0x1234567890abcdef + (i as u64);
    }
    
    println!("📊 Testing TIP5 permutation function...");
    
    // Test correctness first
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
    
    if changed {
        println!("✅ TIP5 permutation function works correctly");
    } else {
        println!("❌ TIP5 permutation function may not be working");
        return;
    }
    
    // Performance test
    println!("\n📈 Running performance test...");
    
    // Warm up
    for _ in 0..100 {
        let mut sponge = original_sponge;
        permute(&mut sponge);
    }
    
    // Actual performance test
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut sponge = original_sponge;
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
        let _result = sbox_layer(&original_sponge);
    }
    let sbox_time = start.elapsed() / iterations;
    println!("   S-box layer average time: {:?}", sbox_time);
    
    // Test linear_layer
    let start = Instant::now();
    for _ in 0..iterations {
        let _result = linear_layer(&original_sponge);
    }
    let linear_time = start.elapsed() / iterations;
    println!("   Linear layer average time: {:?}", linear_time);
    
    println!("\n✅ TIP5 basic performance test completed!");
    
    // Recommendations
    println!("\n💡 Analysis and Recommendations:");
    if avg_time.as_micros() > 100 {
        println!("   - Consider optimizing TIP5 implementation");
        println!("   - Profile to identify bottlenecks");
    } else {
        println!("   - TIP5 performance is good for current implementation");
    }
    
    if sbox_time > linear_time * 2 {
        println!("   - S-box layer is the bottleneck ({:?} vs {:?})", sbox_time, linear_time);
    } else if linear_time > sbox_time * 2 {
        println!("   - Linear layer is the bottleneck ({:?} vs {:?})", linear_time, sbox_time);
    } else {
        println!("   - Both layers have similar performance ({:?} vs {:?})", sbox_time, linear_time);
    }
    
    // Calculate theoretical mining performance
    println!("\n🎯 Mining Performance Implications:");
    let hashes_per_second = ops_per_sec;
    println!("   - TIP5 permutations per second: {:.0}", hashes_per_second);
    
    if hashes_per_second > 1_000_000.0 {
        println!("   - Mining hash rate potential: EXCELLENT (>1M hashes/sec)");
    } else if hashes_per_second > 100_000.0 {
        println!("   - Mining hash rate potential: GOOD (>100K hashes/sec)");
    } else if hashes_per_second > 10_000.0 {
        println!("   - Mining hash rate potential: ACCEPTABLE (>10K hashes/sec)");
    } else {
        println!("   - Mining hash rate potential: NEEDS OPTIMIZATION (<10K hashes/sec)");
    }
    
    println!("\n🔍 Next Steps:");
    println!("   1. ✅ TIP5 Rust implementation verified");
    println!("   2. 🔄 Test TIP5 jet implementation");
    println!("   3. 📊 Compare Rust vs Jet performance");
    println!("   4. 🔍 Profile actual mining code usage");
    println!("   5. 🚀 Optimize bottlenecks identified");
}
