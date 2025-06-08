# 🚀 STARK 验证系统 - 快速开始

## 📋 一分钟上手

### 第一次使用（设置 baseline）

```bash
# 1. 切换到 master 分支
git checkout master

# 2. 生成 master baseline（只需要做一次）
./scripts/generate_proof_json.sh baseline
```

### 日常使用（验证优化）

```bash
# 1. 切换到你的优化分支
git checkout your-optimization-branch

# 2. 一键验证（生成证明 + 验证）
./scripts/quick_verify.sh

# 3. 性能对比
./scripts/compare_performance.sh
```

## 🎯 核心脚本

| 脚本 | 功能 | 用法 |
|------|------|------|
| `generate_proof_json.sh` | 生成证明文件 | `./scripts/generate_proof_json.sh current` |
| `verify_stark_proof.sh` | 验证证明正确性 | `./scripts/verify_stark_proof.sh file.json` |
| `quick_verify.sh` | 一键生成+验证 | `./scripts/quick_verify.sh` |
| `compare_performance.sh` | 性能对比 | `./scripts/compare_performance.sh` |

## 📊 输出文件

```
nockchain/
├── master_baseline/
│   └── master_baseline_small.json          # Master 分支基准
└── current_branch_proofs/
    └── branch_proof_YYYYMMDD_HHMMSS.json   # 带时间戳的分支证明
```

## 🔍 验证结果解读

### 性能对比结果

```bash
📈 性能分析:
   ✅ 性能提升: -2.5s (-1.8%)     # 好！优化有效
   ⚠️  性能下降: +1.2s (+0.9%)     # 需要检查代码
   ➡️  性能相同                    # 无性能影响
```

### 证明一致性检查

```bash
🔍 证明一致性:
   ✅ 证明哈希相同 - 结果一致      # 好！算法正确
   ⚠️  证明哈希不同 - 需要验证     # 需要 STARK 验证
```

## ⚡ 常用命令速查

```bash
# === 快速验证当前分支 ===
./scripts/quick_verify.sh

# === 性能对比 ===
./scripts/compare_performance.sh

# === 生成新证明 ===
./scripts/generate_proof_json.sh current

# === 验证特定文件 ===
./scripts/verify_stark_proof.sh current_branch_proofs/your_file.json

# === 查看所有文件 ===
ls -la current_branch_proofs/
ls -la master_baseline/

# === 比较证明哈希 ===
grep "proof_hash" current_branch_proofs/*.json master_baseline/*.json
```

## 🚨 故障排除

### 问题1: 找不到 baseline 文件
```bash
# 解决方案：生成 master baseline
git checkout master
./scripts/generate_proof_json.sh baseline
```

### 问题2: 证明哈希不同
```bash
# 解决方案：进行 STARK 验证
./scripts/verify_stark_proof.sh your_file.json
```

### 问题3: 验证失败
```bash
# 解决方案：重新生成证明
./scripts/generate_proof_json.sh current
```

## 💡 最佳实践

1. **每次优化后都要验证**
   ```bash
   # 修改代码后
   ./scripts/quick_verify.sh
   ```

2. **定期更新 baseline**
   ```bash
   # master 分支更新后
   git checkout master
   ./scripts/generate_proof_json.sh baseline
   ```

3. **保留历史记录**
   ```bash
   # 文件自动带时间戳，无需手动管理
   ls -t current_branch_proofs/  # 按时间排序查看
   ```

4. **CI/CD 集成**
   ```bash
   # 在 CI 脚本中添加
   ./scripts/quick_verify.sh || exit 1
   ```

## 🎯 典型工作流

### 开发新优化
```bash
git checkout -b my-optimization
# ... 编写优化代码 ...
./scripts/quick_verify.sh
./scripts/compare_performance.sh
```

### 验证现有分支
```bash
git checkout existing-branch
./scripts/quick_verify.sh
```

### 回归测试
```bash
# 生成多个时间点的证明
./scripts/generate_proof_json.sh current
# ... 修改代码 ...
./scripts/generate_proof_json.sh current
# 比较所有文件
./scripts/compare_performance.sh
```

---

## 📚 详细文档

完整使用说明请参考：[STARK_VERIFICATION_README.md](STARK_VERIFICATION_README.md)

---

**🎉 现在你可以放心地进行 STARK 优化，确保性能提升的同时不破坏证明正确性！**
