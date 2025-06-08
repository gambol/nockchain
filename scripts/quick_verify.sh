#!/bin/bash

# 快速验证当前分支的 STARK 证明
# 生成证明并立即验证

set -e

echo "🚀 快速验证当前分支 STARK 证明"
echo "============================="
echo ""

# 检查是否在正确目录
if [ ! -f "Cargo.toml" ]; then
    echo "❌ 错误: 请在 nockchain 项目根目录运行此脚本"
    exit 1
fi

# 显示当前分支
CURRENT_BRANCH=$(git branch --show-current)
echo "📍 当前分支: $CURRENT_BRANCH"
echo ""

# 生成当前分支证明
echo "🔧 步骤 1: 生成当前分支证明..."
./scripts/generate_proof_json.sh current

echo ""
echo "🔍 步骤 2: 查找最新生成的证明文件..."

# 查找最新的证明文件
if [ -d "crates/nockchain/current_branch_proofs" ]; then
    LATEST=$(ls -t crates/nockchain/current_branch_proofs/*.json | head -1)
elif [ -d "current_branch_proofs" ]; then
    LATEST=$(ls -t current_branch_proofs/*.json | head -1)
else
    echo "❌ 错误: 未找到证明文件目录"
    exit 1
fi

if [ -z "$LATEST" ]; then
    echo "❌ 错误: 未找到证明文件"
    exit 1
fi

echo "📊 最新证明文件: $(basename $LATEST)"
echo ""

# 验证证明
echo "🔍 步骤 3: 验证 STARK 证明..."
./scripts/verify_stark_proof.sh "$LATEST"

echo ""
echo "✅ 快速验证完成!"
echo ""
echo "💡 提示:"
echo "   - 证明文件: $LATEST"
echo "   - 如需比较性能，运行: ./scripts/compare_performance.sh"
echo "   - 如需查看所有文件，运行: ls -la current_branch_proofs/"
