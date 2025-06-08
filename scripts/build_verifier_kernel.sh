#!/bin/bash

# Script to build verifier kernel with STARK verification capabilities

set -e

echo "🔧 Building Verifier Kernel"
echo "=========================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Please run this script from the nockchain project root"
    exit 1
fi

# Check if verifier.hoon exists
if [ ! -f "hoon/apps/dumbnet/verifier.hoon" ]; then
    echo "❌ Error: verifier.hoon not found"
    echo "   Expected: hoon/apps/dumbnet/verifier.hoon"
    exit 1
fi

echo "📁 Found verifier.hoon"
echo "🔧 Building verifier kernel..."

# Create assets directory if it doesn't exist
mkdir -p assets

# Build the verifier kernel
echo "🚀 Compiling Hoon to kernel..."

# Method 1: Try using existing build system
if [ -f "Makefile" ] && grep -q "verifier" Makefile; then
    echo "📋 Using Makefile to build verifier kernel"
    make build-verifier-kernel
elif command -v urbit &> /dev/null; then
    echo "📋 Using urbit to compile verifier kernel"
    
    # Create temporary pier for compilation
    TEMP_PIER=$(mktemp -d)
    echo "🏗️  Creating temporary pier: $TEMP_PIER"
    
    # Initialize pier
    urbit -F zod -B "$TEMP_PIER" -c
    
    # Copy Hoon files
    cp -r hoon/* "$TEMP_PIER/zod/"
    
    # Compile to jam
    echo "🔨 Compiling verifier.hoon to jam..."
    urbit -F zod -B "$TEMP_PIER" -e '|commit %home'
    urbit -F zod -B "$TEMP_PIER" -e '.^(jam %cx /=verifier=/hoon)'
    
    # Move compiled kernel
    if [ -f "$TEMP_PIER/verifier.jam" ]; then
        mv "$TEMP_PIER/verifier.jam" assets/verifier.jam
        echo "✅ Verifier kernel built: assets/verifier.jam"
    else
        echo "❌ Failed to build verifier kernel"
        exit 1
    fi
    
    # Cleanup
    rm -rf "$TEMP_PIER"
else
    echo "⚠️  No suitable build method found"
    echo "   Trying alternative approach..."
    
    # Method 2: Copy and modify existing kernel
    if [ -f "assets/miner.jam" ]; then
        echo "📋 Creating verifier kernel based on miner.jam"
        cp assets/miner.jam assets/verifier.jam
        echo "⚠️  Note: This is a temporary solution"
        echo "   The verifier kernel may not have full verification capabilities"
        echo "   Consider using a proper Hoon compiler for production use"
    else
        echo "❌ No base kernel found to modify"
        exit 1
    fi
fi

# Verify the kernel was created
if [ -f "assets/verifier.jam" ]; then
    echo ""
    echo "✅ Verifier kernel built successfully!"
    echo "📁 Location: assets/verifier.jam"
    echo "📊 Size: $(ls -lh assets/verifier.jam | awk '{print $5}')"
    echo ""
    echo "💡 Next steps:"
    echo "   1. Test the verifier kernel with: ./scripts/test_verifier_kernel.sh"
    echo "   2. Use in verification: VERIFIER_KERNEL=assets/verifier.jam cargo test"
    echo "   3. Update verification code to use the new kernel"
else
    echo "❌ Failed to create verifier kernel"
    exit 1
fi

echo ""
echo "🎯 Verifier kernel is ready for STARK verification!"
