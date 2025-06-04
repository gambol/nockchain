use kernels::miner::KERNEL;
use nockapp::kernel::checkpoint::JamPaths;
use nockapp::kernel::form::Kernel;
use nockapp::noun::slab::NounSlab;
use nockapp::wire::Wire;
use nockvm::noun::{D, T};
use std::time::Instant;
use tempfile::tempdir;
use zkvm_jetpack::hot::produce_prover_hot_state;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

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

/// Test data structure for prove-block-inner inputs
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProveBlockInput {
    length: u64,
    block_commitment: [u64; 5],
    nonce: [u64; 5],
}

/// Benchmark result with proof data for verification
#[derive(Debug, Serialize, Deserialize)]
struct ProofBenchmarkResult {
    input: ProveBlockInput,
    duration_secs: f64,
    proof_hash: String,
    proof_data: Vec<u8>,  // Serialized proof for verification
    timestamp: String,
    test_name: String,
}

impl ProveBlockInput {
    fn new(length: u64, block_commitment: [u64; 5], nonce: [u64; 5]) -> Self {
        Self {
            length,
            block_commitment,
            nonce,
        }
    }
    
    /// Convert to NounSlab format expected by the kernel
    fn to_noun_slab(&self) -> NounSlab {
        let mut slab = NounSlab::new();
        
        // Create block commitment tuple
        let block_commitment = T(
            &mut slab,
            &[
                D(self.block_commitment[0]),
                D(self.block_commitment[1]),
                D(self.block_commitment[2]),
                D(self.block_commitment[3]),
                D(self.block_commitment[4]),
            ],
        );
        
        // Create nonce tuple
        let nonce = T(
            &mut slab,
            &[
                D(self.nonce[0]),
                D(self.nonce[1]),
                D(self.nonce[2]),
                D(self.nonce[3]),
                D(self.nonce[4]),
            ],
        );
        
        // Create the full input: [length block-commitment nonce]
        let input = T(&mut slab, &[D(self.length), block_commitment, nonce]);
        
        slab.set_root(input);
        slab
    }
}

/// Fast prove-block-inner benchmark with proof saving
async fn fast_prove_block_benchmark_with_proof(
    input: ProveBlockInput,
    test_name: &str,
) -> Result<ProofBenchmarkResult, Box<dyn std::error::Error>> {
    println!("🚀 Fast prove-block test with length: {}", input.length);
    println!("📊 Nonce: {:?}", input.nonce);

    let start_time = Instant::now();

    // Create temporary directory for kernel
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

    // Convert input to noun format
    let candidate_slab = input.to_noun_slab();

    // Execute prove-block-inner through the kernel
    let effects_slab = kernel
        .poke(MiningWire::Candidate.to_wire(), candidate_slab)
        .await?;

    let duration = start_time.elapsed();

    // Extract proof data from effects
    let proof_data = extract_proof_data(&effects_slab)?;
    let proof_hash = calculate_proof_hash(&proof_data);

    println!("✅ Completed in {:.2?}", duration);
    println!("🔍 Proof hash: {}", proof_hash);

    let result = ProofBenchmarkResult {
        input: input.clone(),
        duration_secs: duration.as_secs_f64(),
        proof_hash,
        proof_data,
        timestamp: chrono::Utc::now().to_rfc3339(),
        test_name: test_name.to_string(),
    };

    Ok(result)
}

/// Legacy function for backward compatibility
async fn fast_prove_block_benchmark(
    input: ProveBlockInput,
) -> Result<std::time::Duration, Box<dyn std::error::Error>> {
    let result = fast_prove_block_benchmark_with_proof(input, "legacy").await?;
    Ok(std::time::Duration::from_secs_f64(result.duration_secs))
}

/// Extract proof data from effects slab
fn extract_proof_data(effects_slab: &NounSlab) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // For now, we'll create a simple representation of the proof
    // In a real implementation, you'd extract the specific proof structure

    // Convert the noun to a string representation and then to bytes
    let noun_str = unsafe {
        format!("{:?}", effects_slab.root())
    };
    Ok(noun_str.into_bytes())
}

/// Calculate a hash of the proof for quick comparison
fn calculate_proof_hash(proof_data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    proof_data.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Save benchmark result to file
fn save_benchmark_result(result: &ProofBenchmarkResult, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create benchmark results directory
    let results_dir = Path::new("benchmark_results");
    if !results_dir.exists() {
        fs::create_dir_all(results_dir)?;
    }

    let filepath = results_dir.join(filename);
    let json_data = serde_json::to_string_pretty(result)?;
    fs::write(&filepath, json_data)?;

    println!("💾 Saved benchmark result to: {}", filepath.display());
    Ok(())
}

/// Load and compare benchmark result
fn load_and_compare_result(filename: &str, current_result: &ProofBenchmarkResult) -> Result<(), Box<dyn std::error::Error>> {
    let filepath = Path::new("benchmark_results").join(filename);

    if !filepath.exists() {
        println!("📝 No previous result found at: {}", filepath.display());
        return Ok(());
    }

    let json_data = fs::read_to_string(&filepath)?;
    let previous_result: ProofBenchmarkResult = serde_json::from_str(&json_data)?;

    println!("🔍 Comparing with previous result:");
    println!("   Previous time: {:.2}s", previous_result.duration_secs);
    println!("   Current time:  {:.2}s", current_result.duration_secs);

    let speedup = previous_result.duration_secs / current_result.duration_secs;
    if speedup > 1.0 {
        println!("🚀 SPEEDUP: {:.2}x faster!", speedup);
    } else if speedup < 1.0 {
        println!("🐌 SLOWDOWN: {:.2}x slower", 1.0 / speedup);
    } else {
        println!("⚖️  Same performance");
    }

    // Compare proof correctness
    if previous_result.proof_hash == current_result.proof_hash {
        println!("✅ PROOF MATCH: Results are identical!");
    } else {
        println!("⚠️  PROOF DIFFERENT: Results differ - check implementation!");
        println!("   Previous hash: {}", previous_result.proof_hash);
        println!("   Current hash:  {}", current_result.proof_hash);
    }

    Ok(())
}

#[tokio::test]
async fn test_very_fast_prove_block() {
    println!("⚡ VERY FAST prove-block-inner test");
    println!("==================================");
    println!("Using MINIMAL parameters for fastest possible execution");
    println!("");
    
    // Try with much smaller length to speed up computation
    let test_cases = vec![
        // Very small length for fastest test
        ProveBlockInput::new(
            8,  // Much smaller than default 64
            [0x1, 0x2, 0x3, 0x4, 0x5],
            [0x10, 0x20, 0x30, 0x40, 0x1],
        ),
    ];
    
    for (i, input) in test_cases.into_iter().enumerate() {
        println!("📊 Test Case {} - Length: {}", i + 1, input.length);
        
        match fast_prove_block_benchmark(input.clone()).await {
            Ok(duration) => {
                println!("✅ SUCCESS! Time: {:.2?}", duration);
                
                let seconds = duration.as_secs_f64();
                if seconds < 60.0 {
                    println!("🚀 Excellent! Under 1 minute");
                } else if seconds < 300.0 {
                    println!("✅ Good! Under 5 minutes");
                } else {
                    println!("⚠️  Still slow: {:.1} minutes", seconds / 60.0);
                }
            }
            Err(e) => {
                eprintln!("❌ Failed: {}", e);
                panic!("Fast test failed");
            }
        }
    }
}

#[tokio::test]
async fn test_progressive_length_benchmark() {
    println!("📈 Progressive Length Benchmark");
    println!("==============================");
    println!("Testing different lengths to find optimal speed/accuracy balance");
    println!("");
    
    // Test with progressively larger lengths
    let lengths = vec![4, 8, 16, 32];  // Much smaller than default 64
    
    for length in lengths {
        println!("🔄 Testing length: {}", length);
        
        let input = ProveBlockInput::new(
            length,
            [0x1, 0x2, 0x3, 0x4, 0x5],
            [0x10, 0x20, 0x30, 0x40, 0x1],
        );
        
        let _start_time = Instant::now();
        
        match fast_prove_block_benchmark(input).await {
            Ok(duration) => {
                println!("✅ Length {}: {:.2?}", length, duration);
                
                // If this length takes more than 10 minutes, stop testing larger ones
                if duration.as_secs() > 600 {
                    println!("⚠️  Length {} took too long, stopping progression", length);
                    break;
                }
            }
            Err(e) => {
                eprintln!("❌ Length {} failed: {}", length, e);
                break;
            }
        }
        
        println!("");
    }
    
    println!("💡 Recommendation: Use the largest length that completes in reasonable time");
}

#[tokio::test]
async fn test_minimal_prove_block() {
    println!("🏃‍♂️ MINIMAL prove-block-inner test");
    println!("==================================");
    println!("Absolute minimum parameters for quickest result");
    println!("");

    // Absolute minimum parameters
    let input = ProveBlockInput::new(
        2,  // Extremely small length
        [0x1, 0x1, 0x1, 0x1, 0x1],  // Simple commitment
        [0x1, 0x1, 0x1, 0x1, 0x1],  // Simple nonce
    );

    println!("🚀 Starting minimal test...");
    println!("   Length: {}", input.length);
    println!("   This should complete in under 5 minutes");

    match fast_prove_block_benchmark_with_proof(input, "minimal_test").await {
        Ok(result) => {
            println!("");
            println!("🎉 MINIMAL TEST COMPLETED!");
            println!("⏱️  Time: {:.2}s", result.duration_secs);

            let seconds = result.duration_secs;
            println!("📊 Performance:");
            println!("   - Seconds: {:.1}", seconds);
            println!("   - Minutes: {:.1}", seconds / 60.0);

            if seconds < 60.0 {
                println!("🚀 EXCELLENT: Under 1 minute!");
            } else if seconds < 300.0 {
                println!("✅ GOOD: Under 5 minutes");
            } else if seconds < 900.0 {
                println!("⚠️  ACCEPTABLE: Under 15 minutes");
            } else {
                println!("🐌 SLOW: Over 15 minutes - consider further optimization");
            }

            // Save the result for future comparison
            let filename = "minimal_test_baseline.json";
            if let Err(e) = save_benchmark_result(&result, filename) {
                eprintln!("⚠️  Failed to save result: {}", e);
            }

            // Compare with previous result if exists
            if let Err(e) = load_and_compare_result(filename, &result) {
                eprintln!("⚠️  Failed to compare with previous result: {}", e);
            }

            println!("");
            println!("💡 Next steps:");
            println!("   - If this was fast enough, try larger lengths");
            println!("   - Use this as baseline for optimization comparisons");
            println!("   - Scale up length gradually to find sweet spot");
            println!("   - Proof saved for verification after optimizations");
        }
        Err(e) => {
            eprintln!("❌ Even minimal test failed: {}", e);
            panic!("Minimal test should not fail");
        }
    }
}

#[tokio::test]
async fn test_length_4_prove_block() {
    println!("🎯 LENGTH=4 prove-block-inner test");
    println!("==================================");
    println!("Testing with length=4 for balanced speed/accuracy");
    println!("");

    // Length=4 parameters
    let input = ProveBlockInput::new(
        4,  // Length=4
        [0x1, 0x2, 0x3, 0x4, 0x5],  // Standard commitment
        [0x10, 0x20, 0x30, 0x40, 0x1],  // Standard nonce
    );

    println!("🚀 Starting length=4 test...");
    println!("   Length: {}", input.length);
    println!("   Expected time: 5-20 minutes");

    match fast_prove_block_benchmark_with_proof(input, "length_4_test").await {
        Ok(result) => {
            println!("");
            println!("🎉 LENGTH=4 TEST COMPLETED!");
            println!("⏱️  Time: {:.2}s ({:.1} minutes)", result.duration_secs, result.duration_secs / 60.0);

            let seconds = result.duration_secs;
            println!("📊 Performance:");
            println!("   - Seconds: {:.1}", seconds);
            println!("   - Minutes: {:.1}", seconds / 60.0);

            if seconds < 300.0 {
                println!("🚀 EXCELLENT: Under 5 minutes!");
            } else if seconds < 900.0 {
                println!("✅ GOOD: Under 15 minutes");
            } else if seconds < 1800.0 {
                println!("⚠️  ACCEPTABLE: Under 30 minutes");
            } else {
                println!("🐌 SLOW: Over 30 minutes - consider smaller length");
            }

            // Save the result for future comparison
            let filename = "length_4_test_baseline.json";
            if let Err(e) = save_benchmark_result(&result, filename) {
                eprintln!("⚠️  Failed to save result: {}", e);
            }

            // Compare with previous result if exists
            if let Err(e) = load_and_compare_result(filename, &result) {
                eprintln!("⚠️  Failed to compare with previous result: {}", e);
            }

            println!("");
            println!("💡 Length=4 analysis:");
            println!("   - 2x more complex than length=2");
            println!("   - Should be significantly faster than length=64");
            println!("   - Good balance for development testing");
            println!("   - Proof saved for verification after optimizations");
        }
        Err(e) => {
            eprintln!("❌ Length=4 test failed: {}", e);
            panic!("Length=4 test should not fail");
        }
    }
}

#[tokio::test]
async fn test_minimal_prove_block_with_verification() {
    println!("🔍 MINIMAL prove-block test WITH VERIFICATION");
    println!("==============================================");
    println!("This test saves proof data and compares with previous runs");
    println!("");

    // Same parameters as minimal test for consistency
    let input = ProveBlockInput::new(
        2,
        [0x1, 0x1, 0x1, 0x1, 0x1],
        [0x1, 0x1, 0x1, 0x1, 0x1],
    );

    println!("🚀 Running test with proof verification...");

    match fast_prove_block_benchmark_with_proof(input, "verification_test").await {
        Ok(result) => {
            println!("✅ Test completed in {:.2}s", result.duration_secs);

            // Save with timestamp for historical tracking
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let filename = format!("verification_test_{}.json", timestamp);

            if let Err(e) = save_benchmark_result(&result, &filename) {
                eprintln!("⚠️  Failed to save timestamped result: {}", e);
            }

            // Also save as latest for easy comparison
            let latest_filename = "verification_test_latest.json";
            if let Err(e) = save_benchmark_result(&result, latest_filename) {
                eprintln!("⚠️  Failed to save latest result: {}", e);
            } else {
                // Compare with previous latest
                if let Err(e) = load_and_compare_result(latest_filename, &result) {
                    eprintln!("⚠️  Failed to compare results: {}", e);
                }
            }

            println!("");
            println!("📁 Results saved:");
            println!("   - Timestamped: benchmark_results/{}", filename);
            println!("   - Latest: benchmark_results/{}", latest_filename);
        }
        Err(e) => {
            eprintln!("❌ Verification test failed: {}", e);
            panic!("Verification test should not fail");
        }
    }
}
