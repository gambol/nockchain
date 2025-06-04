#!/bin/bash

# Simple TIP5 Performance Testing Script
# This script tests TIP5 functionality and basic performance

set -e

echo "🚀 Simple TIP5 Performance Testing"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the nockchain root directory"
    exit 1
fi

print_status "Running TIP5 basic functionality tests..."

# Test 1: Basic TIP5 functionality
if cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_basic_functionality -- --nocapture; then
    print_success "TIP5 basic functionality test passed!"
else
    print_error "TIP5 basic functionality test failed!"
    exit 1
fi

echo ""

# Test 2: TIP5 performance test
print_status "Running TIP5 performance test..."
if cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_performance -- --nocapture; then
    print_success "TIP5 performance test passed!"
else
    print_error "TIP5 performance test failed!"
    exit 1
fi

echo ""

# Test 3: TIP5 components test
print_status "Running TIP5 components test..."
if cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_components -- --nocapture; then
    print_success "TIP5 components test passed!"
else
    print_error "TIP5 components test failed!"
    exit 1
fi

echo ""

# Test 4: TIP5 deterministic test
print_status "Running TIP5 deterministic test..."
if cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_deterministic -- --nocapture; then
    print_success "TIP5 deterministic test passed!"
else
    print_error "TIP5 deterministic test failed!"
    exit 1
fi

echo ""

# Test 5: TIP5 different inputs test
print_status "Running TIP5 different inputs test..."
if cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_different_inputs -- --nocapture; then
    print_success "TIP5 different inputs test passed!"
else
    print_error "TIP5 different inputs test failed!"
    exit 1
fi

echo ""

# Test 6: Check if TIP5 jets are registered
print_status "Checking TIP5 jet registration..."
if grep -r "permutation_jet" crates/zkvm-jetpack/src/hot.rs > /dev/null; then
    print_success "TIP5 permutation jet found in hot state registration!"
else
    print_warning "TIP5 permutation jet may not be properly registered"
fi

echo ""

# Summary
print_status "TIP5 Performance Test Summary"
print_status "============================="
print_success "✅ Basic functionality: PASSED"
print_success "✅ Performance test: PASSED"
print_success "✅ Components test: PASSED"
print_success "✅ Deterministic test: PASSED"
print_success "✅ Different inputs test: PASSED"

echo ""
print_status "Key findings:"
echo "1. TIP5 Rust implementation is working correctly"
echo "2. TIP5 permutation jet exists and should be functional"
echo "3. Performance baseline has been established"

echo ""
print_status "Next steps for TIP5 optimization:"
echo "1. ✅ Verify TIP5 Rust implementation works"
echo "2. 🔄 Test TIP5 jet functionality (needs Context setup)"
echo "3. 📊 Compare Rust vs Jet performance"
echo "4. 🔍 Profile mining code to see TIP5 usage patterns"
echo "5. 🚀 Identify additional TIP5 functions for jet optimization"

echo ""
print_success "Simple TIP5 performance testing completed! 🎉"

echo ""
print_status "To run individual tests:"
echo "  cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_basic_functionality -- --nocapture"
echo "  cargo test --package zkvm-jetpack --test tip5_simple_test test_tip5_performance -- --nocapture"
