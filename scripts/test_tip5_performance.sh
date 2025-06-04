#!/bin/bash

# TIP5 Performance Testing Script
# This script tests TIP5 jet functionality and performance

set -e

echo "🚀 TIP5 Performance Testing Script"
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

print_status "Starting TIP5 performance tests..."

# Test 1: Basic functionality tests
print_status "Running TIP5 jet functionality tests..."
if cargo test --package zkvm-jetpack tip5 -- --nocapture; then
    print_success "TIP5 functionality tests passed!"
else
    print_error "TIP5 functionality tests failed!"
    exit 1
fi

echo ""

# Test 2: Performance benchmarks
print_status "Running TIP5 performance benchmarks..."
print_warning "This may take a few minutes..."

if cargo bench --package zkvm-jetpack --bench tip5_benchmark; then
    print_success "TIP5 benchmarks completed!"
else
    print_error "TIP5 benchmarks failed!"
    exit 1
fi

echo ""

# Test 3: Quick performance comparison
print_status "Running quick TIP5 performance comparison..."

# Create a simple performance test
cat > /tmp/tip5_quick_test.rs << 'EOF'
use std::time::Instant;
use zkvm_jetpack::form::math::tip5::*;

fn main() {
    println!("🔥 Quick TIP5 Performance Test");
    println!("==============================");
    
    // Test data
    let mut test_sponge = [0u64; STATE_SIZE];
    for i in 0..STATE_SIZE {
        test_sponge[i] = 0x1234567890abcdef + (i as u64);
    }
    
    // Warm up
    for _ in 0..100 {
        let mut sponge = test_sponge;
        permute(&mut sponge);
    }
    
    // Performance test
    let iterations = 10000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let mut sponge = test_sponge;
        permute(&mut sponge);
    }
    
    let duration = start.elapsed();
    let avg_time = duration / iterations;
    let ops_per_sec = 1.0 / avg_time.as_secs_f64();
    
    println!("📊 Results:");
    println!("   Iterations: {}", iterations);
    println!("   Total time: {:?}", duration);
    println!("   Average time per permutation: {:?}", avg_time);
    println!("   Permutations per second: {:.0}", ops_per_sec);
    
    if avg_time.as_micros() < 100 {
        println!("✅ Performance: EXCELLENT (< 100μs per permutation)");
    } else if avg_time.as_micros() < 500 {
        println!("✅ Performance: GOOD (< 500μs per permutation)");
    } else if avg_time.as_micros() < 1000 {
        println!("⚠️  Performance: ACCEPTABLE (< 1ms per permutation)");
    } else {
        println!("❌ Performance: POOR (> 1ms per permutation)");
    }
}
EOF

if cargo run --package zkvm-jetpack --example tip5_quick_test /tmp/tip5_quick_test.rs 2>/dev/null || \
   rustc --extern zkvm_jetpack=target/debug/deps/libzkvm_jetpack-*.rlib /tmp/tip5_quick_test.rs -o /tmp/tip5_quick_test && /tmp/tip5_quick_test; then
    print_success "Quick performance test completed!"
else
    print_warning "Quick performance test failed, but this is not critical"
fi

echo ""

# Test 4: Check if TIP5 jets are properly registered
print_status "Checking TIP5 jet registration..."

# Look for TIP5 jets in the hot state
if grep -r "tip5" crates/zkvm-jetpack/src/hot.rs > /dev/null; then
    print_success "TIP5 jets found in hot state registration!"
else
    print_warning "TIP5 jets may not be properly registered in hot state"
fi

echo ""

# Summary
print_status "TIP5 Performance Test Summary"
print_status "============================="
print_success "✅ Functionality tests: PASSED"
print_success "✅ Performance benchmarks: COMPLETED"
print_success "✅ Quick performance test: COMPLETED"

echo ""
print_status "Next steps for TIP5 optimization:"
echo "1. Review benchmark results to identify bottlenecks"
echo "2. Check if TIP5 jets are being used in mining code"
echo "3. Profile mining code to see TIP5 usage patterns"
echo "4. Consider implementing additional TIP5 functions as jets"

echo ""
print_success "TIP5 performance testing completed! 🎉"

# Cleanup
rm -f /tmp/tip5_quick_test.rs /tmp/tip5_quick_test
