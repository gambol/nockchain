# Prove-Block-Inner Benchmark Suite

This document describes how to benchmark the core mining function `prove-block-inner` in nockchain without running the full mining system.

## Overview

The `prove-block-inner` function is the performance bottleneck in nockchain mining. This benchmark suite allows you to:

- Measure the execution time of `prove-block-inner` with controlled inputs
- Compare performance between different implementations
- Profile the STARK proof generation process
- Test optimization improvements

## Function Being Benchmarked

```hoon
++  prove-block-inner
  |=  [length=@ block-commitment=noun-digest:tip5 nonce=noun-digest:tip5]
  ^-  [proof:sp tip5-hash-atom]
  =/  =prove-result:sp
    (prove:np block-commitment nonce length ~)
  ?>  ?=(%& -.prove-result)
  =/  =proof:sp  p.prove-result
  =/  proof-hash=tip5-hash-atom  (proof-to-pow proof)
  [proof proof-hash]
```

## Benchmark Types

### 1. Quick Simulation Benchmark
- **File**: `scripts/benchmark_prove_block.rs`
- **Purpose**: Fast simulation for development
- **Runtime**: ~1 second
- **Use case**: Quick sanity checks

### 2. Integration Test Benchmark
- **File**: `crates/nockchain/tests/prove_block_integration_test.rs`
- **Purpose**: Real kernel execution with controlled inputs
- **Runtime**: Several minutes
- **Use case**: Accurate performance measurement

### 3. Criterion Benchmark
- **File**: `crates/nockchain/benches/prove_block_benchmark.rs`
- **Purpose**: Statistical analysis with multiple runs
- **Runtime**: 10+ minutes
- **Use case**: Detailed performance analysis

## Running Benchmarks

### Quick Start
```bash
# Run all available benchmarks
./scripts/run_prove_block_benchmark.sh

# Run only quick simulation
./scripts/run_prove_block_benchmark.sh quick

# Run only integration test
./scripts/run_prove_block_benchmark.sh integration

# Run only Criterion benchmark
./scripts/run_prove_block_benchmark.sh criterion
```

### Manual Execution

#### Integration Test
```bash
cargo test --test prove_block_integration_test -- --nocapture
```

#### Criterion Benchmark
```bash
cargo bench --bench prove_block_benchmark
```

#### Quick Simulation
```bash
cargo +nightly -Zscript scripts/benchmark_prove_block.rs
```

## Test Inputs

All benchmarks use controlled test inputs:

- **Length**: 64 (standard pow-len)
- **Block Commitment**: `[0x1, 0x2, 0x3, 0x4, 0x5]`
- **Nonces**: Various values like `[0x100, 0x200, 0x300, 0x400, 0x1]`

## Expected Performance

Typical performance characteristics:

- **STARK Proof Generation**: 10-60 seconds per proof
- **Memory Usage**: 1-4 GB during proof generation
- **CPU Usage**: High (single-threaded bottleneck)

## Optimization Areas

Key areas for performance improvement:

1. **Jets Optimization**: Ensure all critical functions have Rust jets
2. **STARK Parameters**: Tune security vs performance trade-offs
3. **Memory Management**: Optimize allocation patterns
4. **Parallelization**: Explore multi-threading opportunities

## Profiling

For detailed profiling:

```bash
# Install flamegraph
cargo install flamegraph

# Profile the integration test
cargo flamegraph --test prove_block_integration_test

# Profile the benchmark
cargo flamegraph --bench prove_block_benchmark
```

## Comparing Implementations

To compare before/after performance:

1. Run benchmark on original code
2. Save results
3. Make optimizations
4. Run benchmark again
5. Compare timing results

Example workflow:
```bash
# Baseline measurement
./scripts/run_prove_block_benchmark.sh integration > baseline_results.txt

# After optimization
./scripts/run_prove_block_benchmark.sh integration > optimized_results.txt

# Compare
diff baseline_results.txt optimized_results.txt
```

## Troubleshooting

### Common Issues

1. **Out of Memory**: Reduce test cases or increase system memory
2. **Slow Performance**: Expected for STARK proofs, consider using quick simulation for development
3. **Build Errors**: Ensure all dependencies are installed

### Debug Mode vs Release Mode

Always run benchmarks in release mode:
```bash
cargo test --release --test prove_block_integration_test
cargo bench --release
```

## Contributing

When adding optimizations:

1. Run baseline benchmark
2. Implement changes
3. Run benchmark again
4. Document performance improvements
5. Include benchmark results in PR

## Files Overview

- `crates/nockchain/benches/prove_block_benchmark.rs` - Criterion benchmark
- `crates/nockchain/tests/prove_block_integration_test.rs` - Integration test
- `scripts/benchmark_prove_block.rs` - Quick simulation
- `scripts/run_prove_block_benchmark.sh` - Benchmark runner script
- `PROVE_BLOCK_BENCHMARK.md` - This documentation
