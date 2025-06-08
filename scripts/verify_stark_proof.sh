#!/bin/bash

# Script to verify STARK proofs from JSON files
# This tool can verify any stored proof data using the Hoon STARK verifier

set -e

echo "🔍 STARK Proof Verification Tool"
echo "================================"
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the nockchain project root"
    exit 1
fi

# Function to verify a specific JSON file
verify_json_file() {
    local json_file="$1"
    
    echo "🔧 Verifying STARK proof from: $json_file"
    echo ""
    
    if [ ! -f "$json_file" ]; then
        echo "❌ Error: JSON file not found: $json_file"
        return 1
    fi
    
    echo "📊 File info:"
    ls -la "$json_file"
    echo ""
    
    echo "🔍 JSON content preview:"
    if command -v jq &> /dev/null; then
        cat "$json_file" | jq -r '
            "Test: " + .test_name +
            "\nSource: " + (.source_branch // "unknown") +
            "\nDuration: " + (.duration_secs | tostring) + "s" +
            "\nProof Hash: " + .proof_hash +
            "\nTimestamp: " + .timestamp +
            "\nInput: length=" + (.input.length | tostring) + 
            ", commitment=" + (.input.block_commitment | tostring) +
            ", nonce=" + (.input.nonce | tostring)'
    else
        echo "Duration: $(grep -o '"duration_secs":[0-9.]*' "$json_file" | cut -d: -f2)s"
        echo "Proof Hash: $(grep -o '"proof_hash":"[^"]*"' "$json_file" | cut -d: -f2 | tr -d '"')"
    fi
    echo ""
    
    echo "🚀 Running STARK verification..."
    start_time=$(date +%s.%N)
    
    VERIFY_JSON_FILE="$json_file" cargo test --test stark_proof_verifier test_verify_proof_from_json_file -- --nocapture
    
    end_time=$(date +%s.%N)
    duration=$(echo "$end_time - $start_time" | bc -l)
    echo ""
    echo "⏱️  Verification completed in ${duration}s"
}

# Parse command line arguments
if [ $# -eq 0 ]; then
    echo "Usage: $0 <json_file> | baseline | help"
    echo ""
    echo "Examples:"
    echo "  $0 crates/nockchain/master_baseline/master_baseline_small.json"
    echo "  $0 benchmark_results/minimal_test_baseline.json"
    echo "  $0 baseline    # Verify master baseline"
    echo "  $0 help        # Show help"
    exit 1
fi

case "$1" in
    "baseline")
        echo "🎯 Verifying master baseline"
        baseline_file="crates/nockchain/master_baseline/master_baseline_small.json"
        
        if [ ! -f "$baseline_file" ]; then
            echo "❌ Master baseline not found: $baseline_file"
            echo ""
            echo "💡 Generate master baseline first:"
            echo "   ./scripts/capture_master_baseline.sh small"
            exit 1
        fi
        
        verify_json_file "$baseline_file"
        ;;
    
    "help"|"-h"|"--help")
        echo "STARK Proof Verification Tool"
        echo "============================="
        echo ""
        echo "This tool verifies STARK proofs stored in JSON files using the Hoon STARK verifier."
        echo ""
        echo "Usage:"
        echo "  $0 <json_file>     - Verify a specific JSON proof file"
        echo "  $0 baseline        - Verify the master baseline"
        echo "  $0 help            - Show this help"
        echo ""
        echo "Examples:"
        echo "  $0 crates/nockchain/master_baseline/master_baseline_small.json"
        echo "  $0 benchmark_results/minimal_test_baseline.json"
        echo "  $0 baseline"
        echo ""
        echo "🔍 What this tool does:"
        echo "  1. Loads proof data from the specified JSON file"
        echo "  2. Extracts the input parameters (length, commitment, nonce)"
        echo "  3. Regenerates the STARK proof using those parameters"
        echo "  4. Calls the Hoon STARK verifier to validate the proof"
        echo "  5. Reports whether the proof passes cryptographic verification"
        echo ""
        echo "📁 Supported JSON formats:"
        echo "  - Master baseline files (from capture_master_baseline.sh)"
        echo "  - Benchmark result files (from run_prove_block_benchmark.sh)"
        echo "  - Any JSON file with the expected proof data structure"
        echo ""
        echo "🎯 Use cases:"
        echo "  - Verify that stored proofs are cryptographically valid"
        echo "  - Check if optimization changes broke proof correctness"
        echo "  - Validate proof data integrity after file transfers"
        echo "  - Debug proof generation issues"
        echo ""
        echo "⚠️  Note: This tool regenerates proofs for verification, so it may take"
        echo "   several minutes to complete depending on the proof parameters."
        ;;
    
    *)
        json_file="$1"
        
        if [ ! -f "$json_file" ]; then
            echo "❌ Error: File not found: $json_file"
            echo ""
            echo "💡 Available files:"
            
            # Look for JSON files in common locations
            if [ -d "crates/nockchain/master_baseline" ]; then
                echo "📁 Master baseline files:"
                ls -la crates/nockchain/master_baseline/*.json 2>/dev/null || echo "   (none found)"
            fi
            
            if [ -d "benchmark_results" ]; then
                echo "📁 Benchmark result files:"
                ls -la benchmark_results/*.json 2>/dev/null || echo "   (none found)"
            fi
            
            if [ -d "crates/nockchain/benchmark_results" ]; then
                echo "📁 Benchmark result files (alt location):"
                ls -la crates/nockchain/benchmark_results/*.json 2>/dev/null || echo "   (none found)"
            fi
            
            exit 1
        fi
        
        verify_json_file "$json_file"
        ;;
esac

echo ""
echo "📋 VERIFICATION SUMMARY"
echo "======================"
echo "✅ STARK proof verification completed"
echo ""
echo "💡 Tips:"
echo "  - Use this tool to verify any stored proof data"
echo "  - Compare verification results between different branches"
echo "  - Ensure optimizations don't break proof correctness"
echo "  - Validate proof data integrity"
