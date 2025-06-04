#!/bin/bash

# Script to run prove-block-inner benchmarks
# This script provides multiple ways to benchmark the core mining function

set -e

echo "🚀 Nockchain prove-block-inner Benchmark Suite"
echo "==============================================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the nockchain project root"
    exit 1
fi

# Function to run with timing
run_with_timing() {
    local name="$1"
    local command="$2"
    
    echo "📊 Running: $name"
    echo "Command: $command"
    echo ""
    
    start_time=$(date +%s.%N)
    eval "$command"
    end_time=$(date +%s.%N)
    
    duration=$(echo "$end_time - $start_time" | bc -l)
    echo ""
    echo "⏱️  Total execution time: ${duration}s"
    echo "----------------------------------------"
    echo ""
}

# Parse command line arguments
BENCHMARK_TYPE="all"
if [ $# -gt 0 ]; then
    BENCHMARK_TYPE="$1"
fi

case "$BENCHMARK_TYPE" in
    "criterion"|"bench")
        echo "🔥 Running Criterion benchmark (most accurate)"
        run_with_timing "Criterion Benchmark" "cargo bench --bench prove_block_benchmark"
        ;;
    
    "test"|"integration")
        echo "🧪 Running integration test (5-15 minutes expected)"
        echo "⚠️  This will run 1 STARK proof generation - please be patient!"
        run_with_timing "Integration Test" "cargo test --test prove_block_integration_test -- --nocapture"
        ;;

    "single"|"one")
        echo "🎯 Running single prove-block test (5-15 minutes expected)"
        echo "⚠️  This runs exactly 1 proof to measure baseline performance"
        run_with_timing "Single Proof Test" "cargo test --test prove_block_quick_test test_single_prove_block_performance -- --nocapture"
        ;;

    "multiple"|"full")
        echo "🔥 Running multiple prove-block tests (15-45 minutes expected)"
        echo "⚠️  WARNING: This runs 3 proofs and takes a LONG time!"
        echo "Press Ctrl+C within 10 seconds to cancel..."
        sleep 10
        run_with_timing "Multiple Proof Test" "cargo test --test prove_block_quick_test test_multiple_prove_block_performance -- --nocapture --ignored"
        ;;
    
    "quick"|"fast")
        echo "⚡ Running quick simulation benchmark"
        if command -v cargo-script &> /dev/null; then
            run_with_timing "Quick Benchmark" "cargo +nightly -Zscript scripts/benchmark_prove_block.rs"
        else
            echo "❌ cargo-script not available, falling back to integration test"
            run_with_timing "Integration Test" "cargo test --test prove_block_integration_test -- --nocapture"
        fi
        ;;

    "minimal"|"tiny")
        echo "🏃‍♂️ Running MINIMAL prove-block test (should complete in <5 minutes)"
        echo "⚡ Using length=2 for fastest possible execution"
        run_with_timing "Minimal Test" "cargo test --test prove_block_fast_test test_minimal_prove_block -- --nocapture"
        ;;

    "progressive"|"scaling")
        echo "📈 Running progressive length benchmark (tests multiple sizes)"
        echo "⚡ Finds optimal length for speed vs accuracy"
        run_with_timing "Progressive Test" "cargo test --test prove_block_fast_test test_progressive_length_benchmark -- --nocapture"
        ;;

    "very-fast"|"vfast")
        echo "⚡ Running very fast prove-block test (length=8)"
        echo "🎯 Should complete much faster than standard test"
        run_with_timing "Very Fast Test" "cargo test --test prove_block_fast_test test_very_fast_prove_block -- --nocapture"
        ;;
    
    "all")
        echo "🎯 Running recommended benchmarks (FAST version)"
        echo "⚠️  This will take 5-20 minutes total"
        echo ""

        # Quick test first
        echo "1️⃣  Quick simulation test (~1 minute)"
        if command -v cargo-script &> /dev/null; then
            run_with_timing "Quick Benchmark" "cargo +nightly -Zscript scripts/benchmark_prove_block.rs"
        else
            echo "⚠️  Skipping quick test (cargo-script not available)"
        fi

        # Fast real test instead of slow one
        echo "2️⃣  Fast real proof test (should be <5 minutes)"
        run_with_timing "Fast Proof Test" "cargo test --test prove_block_fast_test test_very_fast_prove_block -- --nocapture"

        echo "✅ Fast benchmark suite completed!"
        echo "💡 For full testing, use: $0 multiple"
        ;;

    "all-full")
        echo "🔥 Running ALL benchmarks (SLOW version)"
        echo "⚠️  WARNING: This will take 30-60 minutes!"
        echo "Press Ctrl+C within 10 seconds to cancel..."
        sleep 10
        echo ""

        # Quick test first
        echo "1️⃣  Quick simulation test"
        if command -v cargo-script &> /dev/null; then
            run_with_timing "Quick Benchmark" "cargo +nightly -Zscript scripts/benchmark_prove_block.rs"
        else
            echo "⚠️  Skipping quick test (cargo-script not available)"
        fi

        # Multiple integration tests
        echo "2️⃣  Multiple integration tests (15-45 minutes)"
        run_with_timing "Multiple Proof Test" "cargo test --test prove_block_quick_test test_multiple_prove_block_performance -- --nocapture --ignored"

        # Criterion benchmark (if available)
        echo "3️⃣  Criterion benchmark (detailed)"
        if cargo bench --list 2>/dev/null | grep -q "prove_block_benchmark"; then
            run_with_timing "Criterion Benchmark" "cargo bench --bench prove_block_benchmark"
        else
            echo "⚠️  Skipping criterion benchmark (not available)"
        fi
        ;;
    
    "help"|"-h"|"--help")
        echo "Usage: $0 [benchmark_type]"
        echo ""
        echo "Benchmark types:"
        echo "  minimal, tiny       - FASTEST test (length=2, <5 min) [RECOMMENDED FIRST]"
        echo "  very-fast, vfast    - Fast test (length=8, <10 min)"
        echo "  progressive         - Test multiple lengths to find optimal"
        echo "  single, one         - Standard test (length=64, 5-15 min)"
        echo "  multiple, full      - Run 3 standard tests (15-45 min)"
        echo "  quick, fast         - Simulation benchmark (~1 min)"
        echo "  test, integration   - Integration test (5-15 min)"
        echo "  criterion, bench    - Detailed Criterion benchmark"
        echo "  all                 - Run recommended benchmarks (5-20 min) [DEFAULT]"
        echo "  all-full            - Run ALL benchmarks (30-60 min)"
        echo "  help                - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                  # Run recommended benchmarks (fast)"
        echo "  $0 minimal          # FASTEST real test (recommended first time)"
        echo "  $0 very-fast        # Fast test with small length"
        echo "  $0 progressive      # Find optimal length automatically"
        echo "  $0 single           # Standard test (slow but accurate)"
        echo "  $0 quick            # Simulation only (very fast)"
        echo "  $0 multiple         # Multiple tests (very slow)"
        echo ""
        echo "⚠️  TIMING EXPECTATIONS:"
        echo "  - Quick simulation: ~1 minute"
        echo "  - Minimal test (length=2): <5 minutes"
        echo "  - Very fast test (length=8): <10 minutes"
        echo "  - Progressive test: 5-30 minutes (depends on results)"
        echo "  - Standard test (length=64): 5-15 minutes"
        echo "  - Multiple tests: 15-45 minutes"
        echo ""
        echo "🚀 RECOMMENDED FOR FIRST TIME: $0 minimal"
        exit 0
        ;;
    
    *)
        echo "❌ Unknown benchmark type: $BENCHMARK_TYPE"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

echo "✅ Benchmark suite completed!"
echo ""
echo "💡 Tips for optimization:"
echo "   - Check if jets are properly enabled"
echo "   - Monitor CPU and memory usage during benchmarks"
echo "   - Compare results before and after code changes"
echo "   - Use 'cargo flamegraph' for detailed profiling"
