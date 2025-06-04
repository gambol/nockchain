use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use nockvm::interpreter::{Context, NockCancelToken, Slogger};
use nockvm::mem::NockStack;
use nockvm::noun::{Atom, Noun, D, T};
use nockvm::jets::cold::Cold;
use nockvm::jets::warm::Warm;
use nockvm::jets::hot::{Hot, URBIT_HOT_STATE};
use nockvm::hamt::Hamt;
use std::time::Duration;
use std::sync::{Arc, atomic::AtomicIsize};
use zkvm_jetpack::form::math::tip5::*;
use zkvm_jetpack::jets::tip5_jets::*;

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

/// Create a test sponge state with given pattern
fn create_test_sponge(pattern: u8) -> [u64; STATE_SIZE] {
    let mut sponge = [0u64; STATE_SIZE];
    for i in 0..STATE_SIZE {
        sponge[i] = ((pattern as u64) << 56) | ((i as u64) << 48) | 0x123456789abcdef;
    }
    sponge
}

/// Convert array to Hoon list for jet testing
fn array_to_hoon_list(context: &mut Context, values: &[u64; STATE_SIZE]) -> Noun {
    let mut test_list = D(0);
    for &val in values.iter().rev() {
        let atom = Atom::new(&mut context.stack, val).as_noun();
        test_list = T(&mut context.stack, &[atom, test_list]);
    }
    test_list
}

/// Benchmark direct Rust TIP5 permutation
fn bench_rust_permutation(c: &mut Criterion) {
    let mut group = c.benchmark_group("tip5_rust_permutation");
    
    // Test different input patterns
    let patterns = [0u8, 1u8, 0xFF, 0xAA, 0x55];
    
    for pattern in patterns.iter() {
        group.bench_with_input(
            BenchmarkId::new("pattern", format!("0x{:02x}", pattern)),
            pattern,
            |b, &pattern| {
                b.iter(|| {
                    let mut sponge = create_test_sponge(pattern);
                    permute(black_box(&mut sponge));
                    black_box(sponge)
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark TIP5 jet performance
fn bench_jet_permutation(c: &mut Criterion) {
    let mut group = c.benchmark_group("tip5_jet_permutation");

    // Setup context once
    let mut context = create_test_context();
    
    let patterns = [0u8, 1u8, 0xFF, 0xAA, 0x55];
    
    for pattern in patterns.iter() {
        let test_sponge = create_test_sponge(*pattern);
        let test_list = array_to_hoon_list(&mut context, &test_sponge);
        let subject = T(&mut context.stack, &[D(0), test_list]);
        
        group.bench_with_input(
            BenchmarkId::new("pattern", format!("0x{:02x}", pattern)),
            &subject,
            |b, &subject| {
                b.iter(|| {
                    let result = permutation_jet(black_box(&mut context), black_box(subject));
                    black_box(result.unwrap())
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark comparison between Rust and Jet implementations
fn bench_rust_vs_jet_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("tip5_rust_vs_jet");
    group.measurement_time(Duration::from_secs(10));

    let mut context = create_test_context();
    
    let test_sponge = create_test_sponge(0xAA);
    let test_list = array_to_hoon_list(&mut context, &test_sponge);
    let subject = T(&mut context.stack, &[D(0), test_list]);
    
    group.bench_function("rust_direct", |b| {
        b.iter(|| {
            let mut sponge = test_sponge;
            permute(black_box(&mut sponge));
            black_box(sponge)
        });
    });
    
    group.bench_function("jet_call", |b| {
        b.iter(|| {
            let result = permutation_jet(black_box(&mut context), black_box(subject));
            black_box(result.unwrap())
        });
    });
    
    group.finish();
}

/// Benchmark TIP5 components individually
fn bench_tip5_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("tip5_components");
    
    let test_state = create_test_sponge(0x42);
    
    group.bench_function("sbox_layer", |b| {
        b.iter(|| {
            let result = sbox_layer(black_box(&test_state));
            black_box(result)
        });
    });

    group.bench_function("linear_layer", |b| {
        b.iter(|| {
            let result = linear_layer(black_box(&test_state));
            black_box(result)
        });
    });

    group.bench_function("full_round", |b| {
        b.iter(|| {
            let mut sponge = test_state;
            // Simulate one round
            let a = sbox_layer(&sponge);
            let b = linear_layer(&a);
            black_box(b)
        });
    });
    
    group.finish();
}

/// High-throughput test - many permutations
fn bench_tip5_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("tip5_throughput");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(50);
    
    let batch_sizes = [1, 10, 100, 1000];
    
    for &batch_size in batch_sizes.iter() {
        group.bench_with_input(
            BenchmarkId::new("rust_batch", batch_size),
            &batch_size,
            |b, &batch_size| {
                b.iter(|| {
                    for i in 0..batch_size {
                        let mut sponge = create_test_sponge((i % 256) as u8);
                        permute(black_box(&mut sponge));
                        black_box(sponge);
                    }
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_rust_permutation,
    bench_jet_permutation,
    bench_rust_vs_jet_comparison,
    bench_tip5_components,
    bench_tip5_throughput
);
criterion_main!(benches);
