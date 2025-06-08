#!/bin/bash

# STARK Proof Verification System - Quick Start Script
# Usage: ./scripts/quick_start_verification.sh [action]
# Actions: generate, verify, both, clean

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root directory
PROJECT_ROOT="/Volumes/WD/gbwork/nockchain"

echo -e "${BLUE}🚀 STARK Proof Verification System${NC}"
echo -e "${BLUE}===================================${NC}"
echo ""

# Function to print colored output
print_status() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_info() {
    echo -e "${BLUE}📋 $1${NC}"
}

# Function to generate proof data
generate_proof() {
    print_info "Generating STARK proof data..."
    echo ""
    
    cd "$PROJECT_ROOT"
    
    print_info "Running minimal prove-block test (this may take 2-3 minutes)..."
    timeout 300 cargo test --test prove_block_fast_test test_minimal_prove_block_with_extraction -- --nocapture
    
    if [ $? -eq 0 ]; then
        print_status "Proof data generation completed successfully!"
        
        # Find the latest generated file
        LATEST_FILE=$(ls -t crates/nockchain/benchmark_results/minimal_extraction_test_*.json 2>/dev/null | head -1)
        if [ -n "$LATEST_FILE" ]; then
            print_status "Generated file: $LATEST_FILE"
            echo "LATEST_PROOF_FILE=$LATEST_FILE" > /tmp/stark_latest_file
        else
            print_warning "Could not find generated file"
        fi
    else
        print_error "Proof data generation failed!"
        exit 1
    fi
}

# Function to verify proof data
verify_proof() {
    print_info "Verifying STARK proof data..."
    echo ""
    
    cd "$PROJECT_ROOT"
    
    # Check if we have a specific file to verify
    if [ -n "$1" ]; then
        VERIFY_FILE="$1"
        print_info "Verifying specified file: $VERIFY_FILE"
    elif [ -f "/tmp/stark_latest_file" ]; then
        source /tmp/stark_latest_file
        VERIFY_FILE="$LATEST_PROOF_FILE"
        print_info "Verifying latest generated file: $VERIFY_FILE"
    else
        print_info "Using default verification file"
        VERIFY_FILE=""
    fi
    
    # Run verification
    if [ -n "$VERIFY_FILE" ]; then
        VERIFY_FILE="$VERIFY_FILE" timeout 300 cargo test --test stark_proof_verifier test_real_hoon_verification -- --nocapture
    else
        timeout 300 cargo test --test stark_proof_verifier test_real_hoon_verification -- --nocapture
    fi
    
    if [ $? -eq 0 ]; then
        print_status "Proof verification completed successfully!"
        
        # Show latest verification result
        LATEST_RESULT=$(ls -t crates/nockchain/verification_results/real_hoon_verification_*.json 2>/dev/null | head -1)
        if [ -n "$LATEST_RESULT" ]; then
            print_status "Verification result: $LATEST_RESULT"
            
            # Extract verification status
            if command -v jq >/dev/null 2>&1; then
                IS_VALID=$(jq -r '.is_valid' "$LATEST_RESULT" 2>/dev/null)
                DURATION=$(jq -r '.duration_secs' "$LATEST_RESULT" 2>/dev/null)
                
                if [ "$IS_VALID" = "true" ]; then
                    print_status "Verification Status: VALID (${DURATION}s)"
                else
                    print_error "Verification Status: INVALID (${DURATION}s)"
                fi
            fi
        fi
    else
        print_error "Proof verification failed!"
        exit 1
    fi
}

# Function to clean old files
clean_files() {
    print_info "Cleaning old files..."
    
    cd "$PROJECT_ROOT"
    
    # Clean files older than 7 days
    find crates/nockchain/benchmark_results/ -name "*.json" -mtime +7 -delete 2>/dev/null || true
    find crates/nockchain/verification_results/ -name "*.json" -mtime +7 -delete 2>/dev/null || true
    
    print_status "Old files cleaned"
}

# Function to show file status
show_status() {
    print_info "Current file status:"
    echo ""
    
    cd "$PROJECT_ROOT"
    
    echo "📊 Benchmark Results:"
    ls -la crates/nockchain/benchmark_results/*.json 2>/dev/null | tail -3 || echo "  No files found"
    
    echo ""
    echo "🔍 Verification Results:"
    ls -la crates/nockchain/verification_results/*.json 2>/dev/null | tail -3 || echo "  No files found"
}

# Main script logic
case "${1:-both}" in
    "generate")
        generate_proof
        ;;
    "verify")
        if [ -n "$2" ]; then
            verify_proof "$2"
        else
            verify_proof
        fi
        ;;
    "both")
        generate_proof
        echo ""
        verify_proof
        ;;
    "clean")
        clean_files
        ;;
    "status")
        show_status
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [action] [file]"
        echo ""
        echo "Actions:"
        echo "  generate    - Generate new STARK proof data"
        echo "  verify      - Verify STARK proof data"
        echo "  both        - Generate and verify (default)"
        echo "  clean       - Clean old files (>7 days)"
        echo "  status      - Show current file status"
        echo "  help        - Show this help message"
        echo ""
        echo "Examples:"
        echo "  $0                                    # Generate and verify"
        echo "  $0 generate                          # Only generate"
        echo "  $0 verify                            # Verify latest/default file"
        echo "  $0 verify benchmark_results/file.json  # Verify specific file"
        echo "  $0 clean                             # Clean old files"
        echo "  $0 status                            # Show file status"
        ;;
    *)
        print_error "Unknown action: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac

echo ""
print_status "Script completed successfully! 🎉"
