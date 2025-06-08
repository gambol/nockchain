# STARK 证明验证系统使用指南

## 📋 概述

这套工具提供了完整的 STARK 证明生成、存储和验证功能，用于确保代码优化不会破坏 STARK 证明的正确性。

## 🎯 核心概念

### 验证流程
1. **生成 Master Baseline** - 从 master 分支生成标准证明
2. **生成当前分支证明** - 从优化分支生成待验证证明  
3. **STARK 验证** - 验证证明的密码学正确性
4. **对比分析** - 比较性能和正确性

### 文件结构
```
nockchain/
├── scripts/
│   ├── generate_proof_json.sh     # 生成证明 JSON 文件
│   ├── verify_stark_proof.sh      # 验证 STARK 证明
│   └── capture_master_baseline.sh # 捕获 master baseline
├── master_baseline/                # Master 分支基准证明
│   └── master_baseline_small.json
├── current_branch_proofs/          # 当前分支证明文件
│   └── tgryverify_branch_proof_YYYYMMDD_HHMMSS.json
└── crates/nockchain/tests/
    ├── capture_real_stark_proof.rs # 证明生成测试
    └── stark_proof_verifier.rs     # 证明验证测试
```

## 🚀 快速开始

### 1. 生成 Master Baseline（一次性设置）

```bash
# 切换到 master 分支
git checkout master

# 生成 master baseline
./scripts/generate_proof_json.sh baseline
```

**输出**: `master_baseline/master_baseline_small.json`

### 2. 生成当前分支证明

```bash
# 切换到你的优化分支
git checkout your-optimization-branch

# 生成带时间戳的证明文件
./scripts/generate_proof_json.sh current
```

**输出**: `current_branch_proofs/tgryverify_branch_proof_YYYYMMDD_HHMMSS.json`

### 3. 验证证明

```bash
# 验证生成的证明文件
./scripts/verify_stark_proof.sh current_branch_proofs/tgryverify_branch_proof_YYYYMMDD_HHMMSS.json

# 或者验证 master baseline
./scripts/verify_stark_proof.sh baseline
```

## 📚 详细使用说明

### 生成证明文件

#### `./scripts/generate_proof_json.sh`

```bash
# 生成当前分支证明（默认）
./scripts/generate_proof_json.sh
./scripts/generate_proof_json.sh current

# 生成 master baseline
./scripts/generate_proof_json.sh baseline

# 生成两者
./scripts/generate_proof_json.sh both

# 查看帮助
./scripts/generate_proof_json.sh help
```

**特点**:
- 自动添加时间戳到文件名
- 记录分支信息和执行时间
- 包含完整的 STARK 证明数据

### 验证证明文件

#### `./scripts/verify_stark_proof.sh`

```bash
# 验证特定文件
./scripts/verify_stark_proof.sh path/to/proof.json

# 验证 master baseline
./scripts/verify_stark_proof.sh baseline

# 查看帮助
./scripts/verify_stark_proof.sh help
```

**验证过程**:
1. 加载 JSON 文件中的输入参数
2. 重新生成 STARK 证明
3. 调用 Hoon STARK 验证器
4. 报告验证结果

### 捕获 Master Baseline

#### `./scripts/capture_master_baseline.sh`

```bash
# 捕获 master baseline
./scripts/capture_master_baseline.sh small

# 检查已捕获的 baseline
./scripts/capture_master_baseline.sh check

# 清理 baseline
./scripts/capture_master_baseline.sh clean
```

## 🔄 典型工作流程

### 场景1: 验证新的优化

```bash
# 1. 确保有 master baseline
./scripts/generate_proof_json.sh baseline

# 2. 切换到优化分支
git checkout optimization-branch

# 3. 生成优化分支证明
./scripts/generate_proof_json.sh current

# 4. 验证证明正确性
./scripts/verify_stark_proof.sh current_branch_proofs/latest_file.json

# 5. 比较性能
grep "duration_secs" master_baseline/master_baseline_small.json
grep "duration_secs" current_branch_proofs/latest_file.json
```

### 场景2: 回归测试

```bash
# 生成多个时间点的证明
./scripts/generate_proof_json.sh current  # 第一次
# ... 做一些修改 ...
./scripts/generate_proof_json.sh current  # 第二次

# 比较证明哈希
grep "proof_hash" current_branch_proofs/*.json

# 验证所有证明
for file in current_branch_proofs/*.json; do
    echo "验证 $file"
    ./scripts/verify_stark_proof.sh "$file"
done
```

### 场景3: 性能对比

```bash
# 提取性能数据
echo "=== Master Baseline ==="
jq -r '"Duration: " + (.duration_secs | tostring) + "s, Hash: " + .proof_hash' master_baseline/master_baseline_small.json

echo "=== Current Branch ==="
jq -r '"Duration: " + (.duration_secs | tostring) + "s, Hash: " + .proof_hash' current_branch_proofs/latest_file.json
```

## 📊 JSON 文件格式

生成的 JSON 文件包含以下字段：

```json
{
  "input": {
    "length": 2,
    "block_commitment": [1, 1, 1, 1, 1],
    "nonce": [1, 1, 1, 1, 1]
  },
  "duration_secs": 134.99,
  "proof_hash": "ec36a9ebc8d9f010",
  "mining_effects": [...],
  "complete_proof_data": [...],
  "timestamp": "2025-06-08T01:14:04.034799+00:00",
  "test_name": "tgryverify_branch_proof_20250608_011148",
  "source_branch": "tryverify"
}
```

## 🔍 故障排除

### 常见问题

1. **JSON 文件未找到**
   ```bash
   # 检查文件是否存在
   ls -la current_branch_proofs/
   ls -la master_baseline/
   ```

2. **验证失败**
   ```bash
   # 检查证明哈希是否一致
   grep "proof_hash" your_file.json
   
   # 重新生成证明
   ./scripts/generate_proof_json.sh current
   ```

3. **编译错误**
   ```bash
   # 清理并重新编译
   cargo clean
   cargo build
   ```

### 调试模式

```bash
# 使用详细输出
VERIFY_JSON_FILE=your_file.json cargo test --test stark_proof_verifier test_verify_proof_from_json_file -- --nocapture

# 直接运行测试
cargo test --test capture_real_stark_proof test_capture_current_branch_proof -- --nocapture
```

## 💡 最佳实践

1. **定期生成 baseline**: 当 master 分支有重大更新时重新生成
2. **保留历史文件**: 时间戳文件名帮助追踪历史变化
3. **验证关键优化**: 每次重要优化后都要验证证明正确性
4. **性能监控**: 定期比较执行时间，确保优化有效
5. **自动化集成**: 可以将这些脚本集成到 CI/CD 流程中

## 🎯 总结

这套工具确保你的 STARK 优化：
- ✅ **正确性**: 通过密码学验证确保证明有效
- ✅ **可追溯**: 时间戳文件名便于历史追踪  
- ✅ **可比较**: 标准化格式便于性能对比
- ✅ **可重复**: 相同输入产生一致结果

使用这些工具，你可以放心地进行 STARK 优化，确保在提升性能的同时不破坏证明的正确性。

---

## 🚀 快速参考

### 常用命令速查

```bash
# === 生成证明 ===
./scripts/generate_proof_json.sh baseline    # 生成 master baseline
./scripts/generate_proof_json.sh current     # 生成当前分支证明
./scripts/generate_proof_json.sh both        # 生成两者

# === 验证证明 ===
./scripts/verify_stark_proof.sh baseline                    # 验证 baseline
./scripts/verify_stark_proof.sh current_branch_proofs/*.json # 验证当前分支

# === 查看文件 ===
ls -la master_baseline/           # 查看 baseline 文件
ls -la current_branch_proofs/     # 查看当前分支文件

# === 比较性能 ===
grep "duration_secs" master_baseline/*.json current_branch_proofs/*.json
grep "proof_hash" master_baseline/*.json current_branch_proofs/*.json
```

### 一键验证脚本

创建 `quick_verify.sh`:
```bash
#!/bin/bash
echo "🔍 快速验证当前分支"
./scripts/generate_proof_json.sh current
LATEST=$(ls -t current_branch_proofs/*.json | head -1)
echo "📊 验证文件: $LATEST"
./scripts/verify_stark_proof.sh "$LATEST"
```

### 性能对比脚本

创建 `compare_performance.sh`:
```bash
#!/bin/bash
echo "📊 性能对比报告"
echo "==============="
echo "Master Baseline:"
jq -r '"  Duration: " + (.duration_secs | tostring) + "s"' master_baseline/master_baseline_small.json
echo "Current Branch:"
LATEST=$(ls -t current_branch_proofs/*.json | head -1)
jq -r '"  Duration: " + (.duration_secs | tostring) + "s"' "$LATEST"
echo "  File: $(basename $LATEST)"
```
