#!/bin/bash

# 性能对比脚本
# 比较 master baseline 和当前分支的性能

set -e

echo "📊 STARK 证明性能对比报告"
echo "========================="
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

# 检查 master baseline
BASELINE_FILE=""
if [ -f "crates/nockchain/master_baseline/master_baseline_small.json" ]; then
    BASELINE_FILE="crates/nockchain/master_baseline/master_baseline_small.json"
elif [ -f "master_baseline/master_baseline_small.json" ]; then
    BASELINE_FILE="master_baseline/master_baseline_small.json"
else
    echo "❌ 错误: 未找到 master baseline 文件"
    echo "💡 请先运行: ./scripts/generate_proof_json.sh baseline"
    exit 1
fi

# 查找最新的当前分支证明文件
CURRENT_FILE=""
if [ -d "crates/nockchain/current_branch_proofs" ]; then
    CURRENT_FILE=$(ls -t crates/nockchain/current_branch_proofs/*.json 2>/dev/null | head -1)
elif [ -d "current_branch_proofs" ]; then
    CURRENT_FILE=$(ls -t current_branch_proofs/*.json 2>/dev/null | head -1)
fi

if [ -z "$CURRENT_FILE" ]; then
    echo "❌ 错误: 未找到当前分支证明文件"
    echo "💡 请先运行: ./scripts/generate_proof_json.sh current"
    exit 1
fi

echo "📁 文件对比:"
echo "   Master Baseline: $(basename $BASELINE_FILE)"
echo "   Current Branch:  $(basename $CURRENT_FILE)"
echo ""

# 检查是否有 jq 命令
if ! command -v jq &> /dev/null; then
    echo "⚠️  警告: 未安装 jq，使用简化输出"
    echo ""
    echo "📊 Master Baseline:"
    grep -o '"duration_secs":[0-9.]*' "$BASELINE_FILE" | cut -d: -f2 | while read duration; do
        echo "   执行时间: ${duration}s"
    done
    grep -o '"proof_hash":"[^"]*"' "$BASELINE_FILE" | cut -d: -f2 | tr -d '"' | while read hash; do
        echo "   证明哈希: $hash"
    done
    
    echo ""
    echo "📊 Current Branch:"
    grep -o '"duration_secs":[0-9.]*' "$CURRENT_FILE" | cut -d: -f2 | while read duration; do
        echo "   执行时间: ${duration}s"
    done
    grep -o '"proof_hash":"[^"]*"' "$CURRENT_FILE" | cut -d: -f2 | tr -d '"' | while read hash; do
        echo "   证明哈希: $hash"
    done
else
    # 使用 jq 进行详细对比
    echo "📊 Master Baseline:"
    jq -r '"   执行时间: " + (.duration_secs | tostring) + "s"' "$BASELINE_FILE"
    jq -r '"   证明哈希: " + .proof_hash' "$BASELINE_FILE"
    jq -r '"   源分支: " + .source_branch' "$BASELINE_FILE"
    jq -r '"   时间戳: " + .timestamp' "$BASELINE_FILE"
    
    echo ""
    echo "📊 Current Branch:"
    jq -r '"   执行时间: " + (.duration_secs | tostring) + "s"' "$CURRENT_FILE"
    jq -r '"   证明哈希: " + .proof_hash' "$CURRENT_FILE"
    jq -r '"   源分支: " + .source_branch' "$CURRENT_FILE"
    jq -r '"   时间戳: " + .timestamp' "$CURRENT_FILE"
    
    echo ""
    echo "📈 性能分析:"
    
    # 计算性能差异
    BASELINE_DURATION=$(jq -r '.duration_secs' "$BASELINE_FILE")
    CURRENT_DURATION=$(jq -r '.duration_secs' "$CURRENT_FILE")
    
    # 使用 bc 进行浮点数计算（如果可用）
    if command -v bc &> /dev/null; then
        DIFF=$(echo "$CURRENT_DURATION - $BASELINE_DURATION" | bc -l)
        PERCENT=$(echo "scale=2; ($DIFF / $BASELINE_DURATION) * 100" | bc -l)
        
        if (( $(echo "$DIFF > 0" | bc -l) )); then
            echo "   ⚠️  性能下降: +${DIFF}s (+${PERCENT}%)"
        elif (( $(echo "$DIFF < 0" | bc -l) )); then
            DIFF_ABS=$(echo "$DIFF * -1" | bc -l)
            PERCENT_ABS=$(echo "$PERCENT * -1" | bc -l)
            echo "   ✅ 性能提升: -${DIFF_ABS}s (-${PERCENT_ABS}%)"
        else
            echo "   ➡️  性能相同"
        fi
    else
        echo "   📊 Baseline: ${BASELINE_DURATION}s"
        echo "   📊 Current:  ${CURRENT_DURATION}s"
    fi
    
    # 检查证明哈希
    BASELINE_HASH=$(jq -r '.proof_hash' "$BASELINE_FILE")
    CURRENT_HASH=$(jq -r '.proof_hash' "$CURRENT_FILE")
    
    echo ""
    echo "🔍 证明一致性:"
    if [ "$BASELINE_HASH" = "$CURRENT_HASH" ]; then
        echo "   ✅ 证明哈希相同 - 结果一致"
    else
        echo "   ⚠️  证明哈希不同 - 需要验证正确性"
        echo "      Baseline: $BASELINE_HASH"
        echo "      Current:  $CURRENT_HASH"
    fi
fi

echo ""
echo "💡 建议:"
echo "   1. 如果性能提升，验证证明正确性: ./scripts/verify_stark_proof.sh \"$CURRENT_FILE\""
echo "   2. 如果性能下降，检查优化代码"
echo "   3. 如果哈希不同，务必进行 STARK 验证"
echo ""
echo "📁 详细文件:"
echo "   Master: $BASELINE_FILE"
echo "   Current: $CURRENT_FILE"
