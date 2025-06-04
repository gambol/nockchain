#!/bin/bash

# Script to run prove-block-inner benchmarks with proof verification
# This saves proof results and compares them with previous runs

set -e

echo "🔍 Nockchain prove-block-inner Benchmark with Verification"
echo "=========================================================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the nockchain project root"
    exit 1
fi

# Function to run with timing and verification
run_with_verification() {
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
TEST_TYPE="minimal"
if [ $# -gt 0 ]; then
    TEST_TYPE="$1"
fi

case "$TEST_TYPE" in
    "minimal"|"baseline")
        echo "🏃‍♂️ Running MINIMAL test with proof saving"
        echo "⚡ This will save the proof as baseline for future comparisons"
        run_with_verification "Minimal Test with Verification" "cargo test --test prove_block_fast_test test_minimal_prove_block -- --nocapture"
        ;;
    
    "verification"|"verify")
        echo "🔍 Running verification test"
        echo "📊 This compares current results with previous runs"
        run_with_verification "Verification Test" "cargo test --test prove_block_fast_test test_minimal_prove_block_with_verification -- --nocapture"
        ;;
    
    "compare"|"check")
        echo "📋 Checking saved benchmark results"

        # Check multiple possible locations for benchmark results
        RESULTS_DIRS=("benchmark_results" "crates/nockchain/benchmark_results")
        FOUND_DIR=""

        for dir in "${RESULTS_DIRS[@]}"; do
            if [ -d "$dir" ]; then
                FOUND_DIR="$dir"
                break
            fi
        done

        if [ -n "$FOUND_DIR" ]; then
            echo "📁 Found benchmark results directory: $FOUND_DIR"
            ls -la "$FOUND_DIR/"
            echo ""

            # Show latest results if available
            if [ -f "$FOUND_DIR/minimal_test_baseline.json" ]; then
                echo "📊 Latest baseline result:"
                echo "========================="
                if command -v jq &> /dev/null; then
                    cat "$FOUND_DIR/minimal_test_baseline.json" | jq -r '
                        "Test: " + .test_name +
                        "\nTime: " + (.duration_secs | tostring) + "s" +
                        "\nProof Hash: " + .proof_hash +
                        "\nTimestamp: " + .timestamp'
                else
                    echo "Duration: $(grep -o '"duration_secs":[0-9.]*' "$FOUND_DIR/minimal_test_baseline.json" | cut -d: -f2)s"
                    echo "Proof Hash: $(grep -o '"proof_hash":"[^"]*"' "$FOUND_DIR/minimal_test_baseline.json" | cut -d: -f2 | tr -d '"')"
                fi
                echo ""
            fi

            if [ -f "$FOUND_DIR/verification_test_latest.json" ]; then
                echo "🔍 Latest verification result:"
                echo "============================="
                if command -v jq &> /dev/null; then
                    cat "$FOUND_DIR/verification_test_latest.json" | jq -r '
                        "Test: " + .test_name +
                        "\nTime: " + (.duration_secs | tostring) + "s" +
                        "\nProof Hash: " + .proof_hash +
                        "\nTimestamp: " + .timestamp'
                else
                    echo "Duration: $(grep -o '"duration_secs":[0-9.]*' "$FOUND_DIR/verification_test_latest.json" | cut -d: -f2)s"
                    echo "Proof Hash: $(grep -o '"proof_hash":"[^"]*"' "$FOUND_DIR/verification_test_latest.json" | cut -d: -f2 | tr -d '"')"
                fi
                echo ""
            fi
        else
            echo "📝 No benchmark results found. Run a test first:"
            echo "   $0 minimal"
        fi
        ;;
    
    "clean")
        echo "🧹 Cleaning benchmark results"
        if [ -d "benchmark_results" ]; then
            echo "Removing benchmark_results directory..."
            rm -rf benchmark_results
            echo "✅ Cleaned!"
        else
            echo "📝 No benchmark results to clean"
        fi
        ;;
    
    "help"|"-h"|"--help")
        echo "Usage: $0 [test_type]"
        echo ""
        echo "Test types:"
        echo "  minimal, baseline   - Run minimal test and save as baseline"
        echo "  verification, verify - Run verification test with comparison"
        echo "  compare, check      - Show saved benchmark results"
        echo "  clean               - Remove all saved benchmark results"
        echo "  help                - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                  # Run minimal test (default)"
        echo "  $0 minimal          # Run minimal test and save baseline"
        echo "  $0 verification     # Run verification test"
        echo "  $0 compare          # Check saved results"
        echo "  $0 clean            # Clean saved results"
        echo ""
        echo "🔍 Verification Features:"
        echo "  - Saves proof data for correctness verification"
        echo "  - Compares performance with previous runs"
        echo "  - Detects if optimizations change proof results"
        echo "  - Tracks historical performance data"
        echo ""
        echo "📁 Results are saved in: benchmark_results/"
        exit 0
        ;;
    
    *)
        echo "❌ Unknown test type: $TEST_TYPE"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

# Show results summary
echo "📋 VERIFICATION SUMMARY"
echo "======================"

if [ -d "benchmark_results" ]; then
    result_count=$(ls benchmark_results/*.json 2>/dev/null | wc -l)
    echo "📁 Saved results: $result_count files"
    
    if [ $result_count -gt 0 ]; then
        echo "📊 Latest files:"
        ls -t benchmark_results/*.json | head -3 | while read file; do
            echo "   $(basename "$file")"
        done
    fi
else
    echo "📝 No results saved yet"
fi

echo ""
echo "💡 Tips for optimization workflow:"
echo "   1. Run '$0 minimal' to establish baseline"
echo "   2. Make your optimizations"
echo "   3. Run '$0 verification' to check results"
echo "   4. Use '$0 compare' to review all results"
echo "   5. Use '$0 clean' to start fresh if needed"
