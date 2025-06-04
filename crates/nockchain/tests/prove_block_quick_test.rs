use kernels::miner::KERNEL;
use nockapp::kernel::checkpoint::JamPaths;
use nockapp::kernel::form::Kernel;
use nockapp::noun::slab::NounSlab;
use nockapp::wire::Wire;
use nockvm::noun::{D, T};
use std::time::Instant;
use tempfile::tempdir;
use zkvm_jetpack::hot::produce_prover_hot_state;

/// Wire type for mining operations
pub enum MiningWire {
    Candidate,
}

impl Wire for MiningWire {
    const VERSION: u64 = 1;
    const SOURCE: &'static str = "miner";

    fn to_wire(&self) -> nockapp::wire::WireRepr {
        let tags = vec!["candidate".into()];
        nockapp::wire::WireRepr::new(MiningWire::SOURCE, MiningWire::VERSION, tags)
    }
}

/// Create test input for prove-block-inner function
fn create_test_input(nonce_variant: u64) -> NounSlab {
    let mut slab = NounSlab::new();
    
    // Create test parameters: [length block-commitment nonce]
    let length = 64u64; // Standard pow-len
    
    // Test block commitment (5 belt values)
    let block_commitment = T(
        &mut slab,
        &[D(0x1), D(0x2), D(0x3), D(0x4), D(0x5)],
    );
    
    // Test nonce with variant (5 belt values)
    let nonce = T(
        &mut slab,
        &[D(0x100), D(0x200), D(0x300), D(0x400), D(nonce_variant)],
    );
    
    // Create the full input: [length block-commitment nonce]
    let input = T(
        &mut slab,
        &[D(length), block_commitment, nonce],
    );
    
    slab.set_root(input);
    slab
}

/// Single prove-block-inner benchmark
async fn single_prove_block_benchmark(nonce_variant: u64) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    println!("🔄 Testing prove-block-inner with nonce variant: {}", nonce_variant);
    println!("⚠️  This will take 5-15 minutes for STARK proof generation...");
    
    let overall_start = Instant::now();
    
    // Create temporary directory for kernel
    println!("📁 Setting up kernel...");
    let setup_start = Instant::now();
    let snapshot_dir = tempdir()?;
    let hot_state = produce_prover_hot_state();
    let snapshot_path_buf = snapshot_dir.path().to_path_buf();
    let jam_paths = JamPaths::new(snapshot_dir.path());
    
    // Load the mining kernel
    let kernel = Kernel::load_with_hot_state_huge(
        snapshot_path_buf,
        jam_paths,
        KERNEL,
        &hot_state,
        false,
    )
    .await?;
    
    let setup_time = setup_start.elapsed();
    println!("✅ Kernel setup completed in {:.2?}", setup_time);
    
    // Create test input
    let candidate_slab = create_test_input(nonce_variant);
    
    // Execute prove-block-inner through the kernel
    println!("🚀 Starting STARK proof generation...");
    let proof_start = Instant::now();
    
    let _effects_slab = kernel
        .poke(MiningWire::Candidate.to_wire(), candidate_slab)
        .await?;
    
    let proof_time = proof_start.elapsed();
    let total_time = overall_start.elapsed();
    
    println!("✅ STARK proof completed!");
    println!("   Proof generation time: {:.2?}", proof_time);
    println!("   Total time (including setup): {:.2?}", total_time);
    
    Ok(proof_time)
}

#[tokio::test]
async fn test_single_prove_block_performance() {
    println!("🚀 Quick prove-block-inner Performance Test");
    println!("===========================================");
    println!("This test runs only ONE prove-block-inner call to measure performance.");
    println!("Expected time: 5-15 minutes depending on your hardware.");
    println!("");
    
    let nonce_variant = 1u64;
    
    match single_prove_block_benchmark(nonce_variant).await {
        Ok(duration) => {
            println!("");
            println!("📊 PERFORMANCE RESULT");
            println!("====================");
            println!("✅ Test completed successfully!");
            println!("⏱️  Proof generation time: {:.2?}", duration);
            
            // Performance analysis
            let seconds = duration.as_secs_f64();
            println!("📈 Performance metrics:");
            println!("   - Seconds per proof: {:.1}", seconds);
            println!("   - Proofs per hour: {:.1}", 3600.0 / seconds);
            println!("   - Proofs per day: {:.1}", 86400.0 / seconds);
            
            // Performance assessment
            if seconds < 60.0 {
                println!("🚀 Excellent performance! (<1 minute per proof)");
            } else if seconds < 300.0 {
                println!("✅ Good performance! (<5 minutes per proof)");
            } else if seconds < 900.0 {
                println!("⚠️  Moderate performance (5-15 minutes per proof)");
            } else {
                println!("🐌 Slow performance (>15 minutes per proof)");
                println!("   Consider optimizing jets or STARK parameters");
            }
            
            println!("");
            println!("💡 To run multiple tests, uncomment additional test cases in the full test.");
        }
        Err(e) => {
            eprintln!("❌ Test failed: {}", e);
            panic!("Benchmark failed");
        }
    }
}

#[tokio::test]
#[ignore] // Use --ignored to run this test
async fn test_multiple_prove_block_performance() {
    println!("🚀 Multiple prove-block-inner Performance Test");
    println!("==============================================");
    println!("⚠️  WARNING: This test will take 15-45 minutes!");
    println!("Each proof takes 5-15 minutes, and we're running 3 proofs.");
    println!("");
    
    let test_cases = [1u64, 2u64, 3u64];
    let mut results = Vec::new();
    
    for (i, nonce_variant) in test_cases.iter().enumerate() {
        println!("📊 Test Case {} of {}", i + 1, test_cases.len());
        
        match single_prove_block_benchmark(*nonce_variant).await {
            Ok(duration) => {
                results.push(duration);
                println!("✅ Test case {} completed in {:.2?}", i + 1, duration);
            }
            Err(e) => {
                eprintln!("❌ Test case {} failed: {}", i + 1, e);
            }
        }
        
        println!("");
    }
    
    // Analyze results
    if !results.is_empty() {
        let total_time: std::time::Duration = results.iter().sum();
        let avg_time = total_time / results.len() as u32;
        
        println!("📈 FINAL PERFORMANCE SUMMARY");
        println!("============================");
        println!("Successful tests: {}/{}", results.len(), test_cases.len());
        println!("Average time per proof: {:.2?}", avg_time);
        println!("Total testing time: {:.2?}", total_time);
        
        if let (Some(&min), Some(&max)) = (results.iter().min(), results.iter().max()) {
            println!("Fastest proof: {:.2?}", min);
            println!("Slowest proof: {:.2?}", max);
            println!("Performance variance: {:.2?}", max - min);
            
            let variance_percent = ((max - min).as_secs_f64() / avg_time.as_secs_f64()) * 100.0;
            println!("Variance percentage: {:.1}%", variance_percent);
        }
        
        // Performance recommendations
        let avg_seconds = avg_time.as_secs_f64();
        println!("");
        println!("💡 OPTIMIZATION RECOMMENDATIONS");
        println!("===============================");
        if avg_seconds > 600.0 {
            println!("🔥 High priority optimizations needed:");
            println!("   - Check if jets are properly compiled and enabled");
            println!("   - Consider reducing STARK security parameters for testing");
            println!("   - Profile with flamegraph to find bottlenecks");
        } else if avg_seconds > 300.0 {
            println!("⚡ Medium priority optimizations:");
            println!("   - Fine-tune STARK parameters");
            println!("   - Optimize memory allocation patterns");
        } else {
            println!("✅ Performance is reasonable for STARK proofs");
            println!("   - Consider parallelization for multiple proofs");
        }
    }
    
    assert!(!results.is_empty(), "No tests succeeded");
}
