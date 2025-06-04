use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kernels::miner::KERNEL;
use nockapp::kernel::checkpoint::JamPaths;
use nockapp::kernel::form::Kernel;
use nockapp::noun::slab::NounSlab;
use nockapp::wire::Wire;
use nockvm::noun::{D, T};
use nockvm_macros::tas;
use std::time::Duration;
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

/// Benchmark the prove-block-inner function
async fn benchmark_prove_block_inner(nonce_variant: u64) -> Result<(), Box<dyn std::error::Error>> {
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
    
    // Create test input
    let candidate_slab = create_test_input(nonce_variant);
    
    // Call the kernel with the candidate (this will execute prove-block-inner)
    let _effects_slab = kernel
        .poke(MiningWire::Candidate.to_wire(), candidate_slab)
        .await?;
    
    Ok(())
}

/// Synchronous wrapper for the async benchmark
fn sync_benchmark_prove_block_inner(nonce_variant: u64) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        benchmark_prove_block_inner(nonce_variant)
            .await
            .expect("Benchmark failed")
    });
}

/// Criterion benchmark function
fn prove_block_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("prove_block_inner");
    
    // Set longer measurement time since STARK proving is slow
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10); // Fewer samples due to long execution time
    
    // Benchmark with different nonce values
    for nonce_variant in [1u64, 2u64, 3u64] {
        group.bench_with_input(
            format!("nonce_{}", nonce_variant),
            &nonce_variant,
            |b, &nonce| {
                b.iter(|| {
                    sync_benchmark_prove_block_inner(black_box(nonce));
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, prove_block_benchmark);
criterion_main!(benches);
