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

/// Test data structure for prove-block-inner inputs
#[derive(Debug, Clone)]
struct ProveBlockInput {
    length: u64,
    block_commitment: [u64; 5],
    nonce: [u64; 5],
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

/// Result of a prove-block-inner benchmark
#[derive(Debug)]
struct BenchmarkResult {
    input: ProveBlockInput,
    duration: std::time::Duration,
    success: bool,
    error: Option<String>,
}

/// Main benchmark function for prove-block-inner
async fn benchmark_prove_block_inner(
    input: ProveBlockInput,
) -> Result<BenchmarkResult, Box<dyn std::error::Error>> {
    println!("🔄 Testing with nonce: {:?}", input.nonce);
    
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
    let result = kernel
        .poke(MiningWire::Candidate.to_wire(), candidate_slab)
        .await;
    
    let duration = start_time.elapsed();
    
    match result {
        Ok(_effects) => {
            println!("✅ Success in {:.2?}", duration);
            Ok(BenchmarkResult {
                input,
                duration,
                success: true,
                error: None,
            })
        }
        Err(e) => {
            println!("❌ Failed in {:.2?}: {}", duration, e);
            Ok(BenchmarkResult {
                input,
                duration,
                success: false,
                error: Some(e.to_string()),
            })
        }
    }
}

#[tokio::test]
async fn test_prove_block_inner_performance() {
    println!("🚀 Starting prove-block-inner performance test");
    
    // Define test cases with different inputs
    // REDUCED: Only 1 test case to speed up benchmarking
    let test_cases = vec![
        ProveBlockInput::new(
            64,
            [0x1, 0x2, 0x3, 0x4, 0x5],
            [0x100, 0x200, 0x300, 0x400, 0x1],
        ),
        // Uncomment these for full testing (each takes 5-10 minutes)
        // ProveBlockInput::new(
        //     64,
        //     [0x1, 0x2, 0x3, 0x4, 0x5],
        //     [0x100, 0x200, 0x300, 0x400, 0x2],
        // ),
        // ProveBlockInput::new(
        //     64,
        //     [0x1, 0x2, 0x3, 0x4, 0x5],
        //     [0x100, 0x200, 0x300, 0x400, 0x3],
        // ),
    ];
    
    let mut results = Vec::new();
    
    for (i, input) in test_cases.into_iter().enumerate() {
        println!("\n📊 Test Case {}", i + 1);
        
        match benchmark_prove_block_inner(input).await {
            Ok(result) => results.push(result),
            Err(e) => {
                eprintln!("❌ Test case {} failed: {}", i + 1, e);
            }
        }
    }
    
    // Analyze results
    if !results.is_empty() {
        let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();
        
        if !successful_results.is_empty() {
            let total_time: std::time::Duration = successful_results.iter().map(|r| r.duration).sum();
            let avg_time = total_time / successful_results.len() as u32;
            
            println!("\n📈 PERFORMANCE SUMMARY");
            println!("======================");
            println!("Successful tests: {}/{}", successful_results.len(), results.len());
            println!("Average time: {:.2?}", avg_time);
            println!("Total time: {:.2?}", total_time);
            
            if let (Some(min), Some(max)) = (
                successful_results.iter().map(|r| r.duration).min(),
                successful_results.iter().map(|r| r.duration).max(),
            ) {
                println!("Fastest: {:.2?}", min);
                println!("Slowest: {:.2?}", max);
                println!("Variance: {:.2?}", max - min);
            }
        }
    }
    
    // At least one test should succeed
    assert!(results.iter().any(|r| r.success), "No tests succeeded");
}
