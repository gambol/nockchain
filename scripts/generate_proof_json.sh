#!/bin/bash

# Script to generate proof JSON files with timestamps
# This creates proof files that can be used for verification

set -e

echo "🔧 Proof JSON Generator"
echo "======================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the nockchain project root"
    exit 1
fi

# Get current branch name
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 Current branch: $CURRENT_BRANCH"

# Generate timestamp
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
echo "🕒 Timestamp: $TIMESTAMP"
echo ""

# Function to generate current branch proof
generate_current_branch_proof() {
    echo "🔧 Generating current branch proof JSON..."
    echo "Command: cargo test --test capture_real_stark_proof test_capture_current_branch_proof -- --nocapture"
    echo ""
    
    start_time=$(date +%s.%N)
    cargo test --test capture_real_stark_proof test_capture_current_branch_proof -- --nocapture
    end_time=$(date +%s.%N)
    
    duration=$(echo "$end_time - $start_time" | bc -l)
    echo ""
    echo "⏱️  Total generation time: ${duration}s"
    echo "----------------------------------------"
    echo ""
}

# Function to generate master baseline
generate_master_baseline() {
    echo "🔧 Generating master baseline JSON..."
    echo "Command: cargo test --test capture_real_stark_proof test_capture_master_baseline_small -- --nocapture"
    echo ""
    
    start_time=$(date +%s.%N)
    cargo test --test capture_real_stark_proof test_capture_master_baseline_small -- --nocapture
    end_time=$(date +%s.%N)
    
    duration=$(echo "$end_time - $start_time" | bc -l)
    echo ""
    echo "⏱️  Total generation time: ${duration}s"
    echo "----------------------------------------"
    echo ""
}

# Parse command line arguments
PROOF_TYPE="current"
if [ $# -gt 0 ]; then
    PROOF_TYPE="$1"
fi

case "$PROOF_TYPE" in
    "current"|"branch")
        echo "🏃‍♂️ Generating CURRENT BRANCH proof JSON"
        echo "⚡ This will create a timestamped proof file for verification"
        echo "📁 Output directory: current_branch_proofs/"
        echo "📄 Filename format: tgryverify_branch_proof_${TIMESTAMP}.json"
        echo ""
        generate_current_branch_proof
        ;;

    "baseline"|"master")
        echo "🏃‍♂️ Generating MASTER BASELINE proof JSON"
        echo "⚡ This will create the gold standard baseline"
        echo "📁 Output directory: master_baseline/"
        echo "📄 Filename: master_baseline_small.json"
        echo ""
        generate_master_baseline
        ;;
    
    "both")
        echo "🏃‍♂️ Generating BOTH baseline and current branch proofs"
        echo ""
        echo "1️⃣ First generating master baseline..."
        generate_master_baseline
        echo ""
        echo "2️⃣ Now generating current branch proof..."
        generate_current_branch_proof
        ;;
    
    "help"|"-h"|"--help")
        echo "Usage: $0 [type]"
        echo ""
        echo "Types:"
        echo "  current, branch     - Generate current branch proof with timestamp (default)"
        echo "  baseline, master    - Generate master baseline proof"
        echo "  both                - Generate both baseline and current branch proofs"
        echo "  help                - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                  # Generate current branch proof (default)"
        echo "  $0 current          # Generate current branch proof"
        echo "  $0 baseline         # Generate master baseline"
        echo "  $0 both             # Generate both"
        echo ""
        echo "🎯 Purpose:"
        echo "  This tool generates proof JSON files that can be used for:"
        echo "  - STARK proof verification"
        echo "  - Performance comparison"
        echo "  - Optimization validation"
        echo "  - Regression testing"
        echo ""
        echo "📁 Output directories:"
        echo "  - current_branch_proofs/  (timestamped files)"
        echo "  - master_baseline/        (baseline files)"
        echo ""
        echo "💡 Workflow:"
        echo "  1. Generate baseline: $0 baseline"
        echo "  2. Make optimizations"
        echo "  3. Generate current proof: $0 current"
        echo "  4. Verify: ./scripts/verify_stark_proof.sh current_branch_proofs/filename.json"
        exit 0
        ;;
    
    *)
        echo "❌ Unknown type: $PROOF_TYPE"
        echo "Run '$0 help' for usage information"
        exit 1
        ;;
esac

# Show results summary
echo "📋 PROOF GENERATION SUMMARY"
echo "==========================="

case "$PROOF_TYPE" in
    "current"|"branch")
        if [ -d "current_branch_proofs" ]; then
            proof_count=$(ls current_branch_proofs/*.json 2>/dev/null | wc -l)
            echo "📁 Current branch proofs: $proof_count files"
            
            if [ $proof_count -gt 0 ]; then
                echo "📊 Latest proof files:"
                ls -t current_branch_proofs/*.json | head -3 | while read file; do
                    echo "   $(basename "$file")"
                done
            fi
        else
            echo "📝 No current branch proofs generated yet"
        fi
        ;;
        
    "baseline"|"master")
        if [ -d "master_baseline" ]; then
            baseline_count=$(ls master_baseline/*.json 2>/dev/null | wc -l)
            echo "📁 Master baseline files: $baseline_count files"
            
            if [ $baseline_count -gt 0 ]; then
                echo "📊 Baseline files:"
                ls master_baseline/*.json | while read file; do
                    echo "   $(basename "$file")"
                done
            fi
        else
            echo "📝 No master baseline generated yet"
        fi
        ;;
        
    "both")
        echo "📁 Generated both baseline and current branch proofs"
        if [ -d "master_baseline" ]; then
            echo "   Master baseline: $(ls master_baseline/*.json 2>/dev/null | wc -l) files"
        fi
        if [ -d "current_branch_proofs" ]; then
            echo "   Current branch: $(ls current_branch_proofs/*.json 2>/dev/null | wc -l) files"
        fi
        ;;
esac

echo ""
echo "💡 Next steps:"
echo "   1. Verify generated proofs with: ./scripts/verify_stark_proof.sh <filename>"
echo "   2. Compare proofs between branches"
echo "   3. Use for optimization validation"
echo ""
echo "🎯 Generated files are ready for STARK verification!"
