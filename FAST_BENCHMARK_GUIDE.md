# 🚀 Fast Prove-Block-Inner Benchmark Guide

## Problem: Standard Tests Too Slow

The standard `prove-block-inner` test with `length=64` takes **1+ hours** to complete, which is too slow for development and optimization work.

## Solution: Reduced Length Parameters

By reducing the `length` parameter, we can dramatically speed up the STARK proof generation while still getting meaningful performance measurements.

## 📊 Speed vs Accuracy Trade-offs

| Length | Expected Time | Use Case |
|--------|---------------|----------|
| 2      | <5 minutes    | 🚀 **Fastest baseline** |
| 8      | <10 minutes   | ⚡ **Development testing** |
| 16     | <20 minutes   | 🎯 **Balanced testing** |
| 32     | <45 minutes   | 📈 **Detailed analysis** |
| 64     | 1+ hours      | 🔬 **Production accuracy** |

## 🎯 Recommended Testing Strategy

### 1. **First Time** - Get Quick Baseline
```bash
./scripts/run_prove_block_benchmark.sh minimal
```
- Uses `length=2`
- Should complete in <5 minutes
- Gives you a baseline performance number

### 2. **Development** - Fast Iteration
```bash
./scripts/run_prove_block_benchmark.sh very-fast
```
- Uses `length=8`
- Should complete in <10 minutes
- Good for testing optimizations

### 3. **Find Optimal** - Automatic Scaling
```bash
./scripts/run_prove_block_benchmark.sh progressive
```
- Tests lengths: 4, 8, 16, 32
- Stops when time becomes too long
- Finds your system's sweet spot

### 4. **Production** - Full Accuracy
```bash
./scripts/run_prove_block_benchmark.sh single
```
- Uses `length=64` (standard)
- Takes 1+ hours but gives production-accurate results

## 🔧 What Length Parameter Controls

The `length` parameter in `prove-block-inner` controls:
- **Nock computation complexity**: Larger length = more complex Nock program
- **STARK proof size**: More computation = larger proof
- **Proof generation time**: Exponentially increases with length

## 📈 Performance Scaling

Based on STARK complexity, proof time roughly scales as:
- `length=2`: ~1x baseline
- `length=8`: ~4x baseline  
- `length=16`: ~16x baseline
- `length=32`: ~64x baseline
- `length=64`: ~256x baseline

## 🎯 Optimization Workflow

1. **Baseline**: Run `minimal` test, record time
2. **Optimize**: Make code changes
3. **Test**: Run `minimal` test again
4. **Compare**: Calculate speedup percentage
5. **Validate**: Run `very-fast` test to confirm
6. **Final**: Run `single` test for production validation

## 🚨 Important Notes

### ✅ What These Tests Measure
- STARK proof generation performance
- Jets optimization effectiveness
- Memory allocation efficiency
- Core computational bottlenecks

### ⚠️ What They Don't Measure
- Network mining difficulty
- Real mining profitability
- Full system integration
- Production mining loops

### 🎯 Perfect for Optimization
- Comparing before/after performance
- Testing jets improvements
- Profiling bottlenecks
- Development iteration

## 🔍 Monitoring Long Tests

If you do run longer tests, monitor progress:
```bash
# In another terminal
./scripts/monitor_benchmark.sh monitor
```

This shows:
- CPU usage (should be ~100% during proof generation)
- Memory usage
- Process status
- Time estimates

## 🚀 Quick Start Commands

```bash
# Absolute fastest (recommended first)
./scripts/run_prove_block_benchmark.sh minimal

# Fast development testing
./scripts/run_prove_block_benchmark.sh very-fast

# Find optimal length for your system
./scripts/run_prove_block_benchmark.sh progressive

# Standard accuracy (slow)
./scripts/run_prove_block_benchmark.sh single
```

## 💡 Pro Tips

1. **Start Small**: Always begin with `minimal` to ensure everything works
2. **Iterate Fast**: Use `very-fast` for development and optimization
3. **Scale Up**: Use `progressive` to find your system's limits
4. **Validate**: Use `single` only for final validation
5. **Monitor**: Use the monitor script for long-running tests

## 🎯 Expected Results

On a typical development machine:
- **Minimal (length=2)**: 2-5 minutes
- **Very Fast (length=8)**: 5-15 minutes  
- **Progressive**: 10-30 minutes total
- **Standard (length=64)**: 1-3 hours

Your results will vary based on:
- CPU performance
- Available memory
- Jets optimization level
- System load
