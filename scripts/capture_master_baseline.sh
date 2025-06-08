#!/bin/bash

# Script to capture master branch baseline for optimization verification
# This creates the gold standard that all optimizations will be verified against

set -e

echo "🎯 Master Baseline Capture Tool"
echo "==============================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the nockchain project root"
    exit 1
fi

# Check if we're on the right branch
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"

if [ "$CURRENT_BRANCH" != "tgryverify" ]; then
    echo "⚠️  Warning: You're not on the tgryverify branch"
    echo "   This script is designed to run on the tgryverify branch"
    echo "   which should be based on master for clean baseline capture"
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

echo ""
echo "🔍 This will capture a master branch baseline for:"
echo "   - STARK proof verification"
echo "   - Performance comparison"
echo "   - Optimization validation"
echo ""

# Function to run baseline capture
run_baseline_capture() {
    local test_type="$1"
    
    echo "📊 Capturing master baseline: $test_type"
    echo "Command: cargo test --test capture_real_stark_proof test_capture_master_baseline_small -- --nocapture"
    echo ""
    
    start_time=$(date +%s.%N)
    cargo test --test capture_real_stark_proof test_capture_master_baseline_small -- --nocapture
    end_time=$(date +%s.%N)
    
    duration=$(echo "$end_time - $start_time" | bc -l)
    echo ""
    echo "⏱️  Total capture time: ${duration}s"
    echo "----------------------------------------"
    echo ""
}

# Parse command line arguments
TEST_TYPE="small"
if [ $# -gt 0 ]; then
    TEST_TYPE="$1"
fi

case "$TEST_TYPE" in
    "small"|"baseline")
        echo "🏃‍♂️ Capturing SMALL baseline (length=2)"
        echo "⚡ This will be fast and suitable for optimization testing"
        run_baseline_capture "Small Baseline"
        ;;

    "check"|"verify")
        echo "📋 Checking captured baselines"

        # Check if baseline directory exists
        if [ -d "master_baseline" ]; then
            echo "📁 Found master baseline directory: master_baseline/"
            ls -la master_baseline/
            echo ""

            # Show baseline info if available
            if [ -f "master_baseline/master_baseline_small.json" ]; then
                echo "📊 Master baseline (small):"
                echo "=========================="
                if command -v jq &> /dev/null; then
                    cat master_baseline/master_baseline_small.json | jq -r '
                        "Source Branch: " + .source_branch +
                        "\nTest: " + .test_name +
                        "\nDuration: " + (.duration_secs | tostring) + "s" +
                        "\nProof Hash: " + .proof_hash +
                        "\nTimestamp: " + .timestamp +
                        "\nInput: length=" + (.input.length | tostring) + 
                        ", commitment=" + (.input.block_commitment | tostring) +
                        ", nonce=" + (.input.nonce | tostring)'
                else
                    echo "Duration: $(grep -o '"duration_secs":[0-9.]*' master_baseline/master_baseline_small.json | cut -d: -f2)s"
                    echo "Proof Hash: $(grep -o '"proof_hash":"[^"]*"' master_baseline/master_baseline_small.json | cut -d: -f2 | tr -d '"')"
                    echo "Source: $(grep -o '"source_branch":"[^"]*"' master_baseline/master_baseline_small.json | cut -d: -f2 | tr -d '"')"
                fi
                echo ""
            fi
        else
            echo "📝 No master baseline found. Run '$0 small' to capture baseline"
        fi
        ;;
    
    "clean")
        echo "🧹 Cleaning master baseline"
        if [ -d "master_baseline" ]; then
            echo "Removing master_baseline directory..."
            rm -rf master_baseline
            echo "✅ Cleaned!"
        else
            echo "📝 No master baseline to clean"
        fi
        ;;
    
    "help"|"-h"|"--help")
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  small, baseline     - Capture master baseline with small parameters (default)"
        echo "  check, verify       - Show captured baseline information"
        echo "  clean               - Remove all captured baselines"
        echo "  help                - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                  # Capture small baseline (default)"
        echo "  $0 small            # Capture small baseline"
        echo "  $0 check            # Check captured baselines"
        echo "  $0 clean            # Clean captured baselines"
        echo ""
        echo "🎯 Purpose:"
        echo "  This tool captures a master branch baseline that serves as the"
        echo "  gold standard for verifying optimizations. The baseline contains:"
        echo "  - Real STARK proof data from master branch"
        echo "  - Performance metrics"
        echo "  - Complete computational results"
        echo ""
        echo "📁 Output: master_baseline/"
        echo ""
        echo "💡 Workflow:"
        echo "  1. Run '$0 small' to capture master baseline"
        echo "  2. Switch to optimization branch"
        echo "  3. Run optimization verification against this baseline"
        echo "  4. Ensure optimizations don't break STARK correctness"
        exit 0
        ;;
    
    *)
        echo "❌ Unknown command: $TEST_TYPE"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

# Show results summary
echo "📋 BASELINE CAPTURE SUMMARY"
echo "==========================="

if [ -d "master_baseline" ]; then
    baseline_count=$(ls master_baseline/*.json 2>/dev/null | wc -l)
    echo "📁 Captured baselines: $baseline_count files"
    
    if [ $baseline_count -gt 0 ]; then
        echo "📊 Available baselines:"
        ls -t master_baseline/*.json | while read file; do
            echo "   $(basename "$file")"
        done
    fi
else
    echo "📝 No baselines captured yet"
fi

echo ""
echo "💡 Next steps:"
echo "   1. Verify baseline with: $0 check"
echo "   2. Switch to optimization branch"
echo "   3. Run optimization verification against this baseline"
echo "   4. Use this baseline as gold standard for all optimizations"
echo ""
echo "🎯 This baseline ensures your optimizations maintain STARK correctness!"
