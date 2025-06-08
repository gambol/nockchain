# STARK验证系统 - 快速开始指南

## 🚀 一分钟快速开始

### 方法1：使用自动化脚本（推荐）

```bash
# 进入项目目录
cd /Volumes/WD/gbwork/nockchain

# 运行自动化脚本（生成+验证）
./crates/nockchain/scripts/quick_start_verification.sh
```

### 方法2：手动执行

```bash
# 1. 生成证明数据（2-3分钟）
timeout 300 cargo test --test prove_block_fast_test test_minimal_prove_block_with_extraction -- --nocapture

# 2. 验证证明数据（1秒）
timeout 300 cargo test --test stark_proof_verifier test_real_hoon_verification -- --nocapture
```

## 📋 常用命令

### 🔧 自动化脚本用法

```bash
# 查看帮助
./crates/nockchain/scripts/quick_start_verification.sh help

# 只生成证明数据
./crates/nockchain/scripts/quick_start_verification.sh generate

# 只验证数据
./crates/nockchain/scripts/quick_start_verification.sh verify

# 验证特定文件
./crates/nockchain/scripts/quick_start_verification.sh verify benchmark_results/your_file.json

# 查看文件状态
./crates/nockchain/scripts/quick_start_verification.sh status

# 清理旧文件
./crates/nockchain/scripts/quick_start_verification.sh clean
```

### 🔍 手动验证特定文件

```bash
# 验证指定文件
VERIFY_FILE=benchmark_results/minimal_extraction_test_20250608_055801.json timeout 300 cargo test --test stark_proof_verifier test_real_hoon_verification -- --nocapture

# 查看可用文件
ls -la crates/nockchain/benchmark_results/*.json
```

## 📊 预期输出

### 生成证明数据时：
```
✅ Successfully extracted STARK proof data
   Proof objects: 53
   Proof hashes: 0
✅ Completed in 134.91s
🔍 Proof hash: 45205e5d350e0d145fdb4f7f9f2d0710bd178e87e235064f36e8639d872be569f2e13a894677a776
💾 JSON file created: benchmark_results/minimal_extraction_test_20250608_055801.json
```

### 验证证明数据时：
```
🧪 Testing Real Hoon STARK Verification
📁 Found test file: benchmark_results/minimal_extraction_test_20250608_055801.json
🔐 Performing cryptographic STARK verification...
✅ All cryptographic checks passed
   Status: ✅ VALID
   Duration: 0.001s
   Proof objects: 53
💾 Result saved to: verification_results/real_hoon_verification_20250608_073615.json
```

## 🔍 验证篡改检测

测试验证器是否能检测数据篡改：

```bash
# 1. 复制一个文件
cp crates/nockchain/benchmark_results/minimal_extraction_test_20250608_055801.json crates/nockchain/benchmark_results/test_tampered.json

# 2. 编辑文件，修改任何heights数据（例如把 [1,2,3] 改为 [1,2,7]）
nano crates/nockchain/benchmark_results/test_tampered.json

# 3. 验证篡改文件
VERIFY_FILE=benchmark_results/test_tampered.json timeout 300 cargo test --test stark_proof_verifier test_real_hoon_verification -- --nocapture
```

**预期结果**：验证失败，显示 `❌ INVALID`

## 📁 文件位置

- **证明数据**：`crates/nockchain/benchmark_results/`
- **验证结果**：`crates/nockchain/verification_results/`
- **脚本**：`crates/nockchain/scripts/`
- **文档**：`crates/nockchain/README_STARK_VERIFICATION.md`

## ⚡ 性能数据

- **证明生成**：~135秒（2.25分钟）
- **数据验证**：~1毫秒
- **文件大小**：~12KB
- **Proof对象**：53个（49个heights + 4个merkle roots）

## 🛠️ 故障排除

### 常见问题：

1. **权限错误**：
```bash
chmod +x crates/nockchain/scripts/quick_start_verification.sh
```

2. **文件未找到**：
```bash
ls -la crates/nockchain/benchmark_results/
```

3. **编译错误**：
```bash
cargo clean
cargo build --test prove_block_fast_test
```

4. **超时问题**：
```bash
# 增加timeout时间
timeout 600 cargo test ...
```

## 🎯 实际应用场景

### 优化工作流程：

1. **生成基准数据**：
```bash
./crates/nockchain/scripts/quick_start_verification.sh generate
```

2. **修改代码**（实施优化）

3. **生成优化后数据**：
```bash
./crates/nockchain/scripts/quick_start_verification.sh generate
```

4. **验证正确性**：
```bash
./crates/nockchain/scripts/quick_start_verification.sh verify
```

5. **比较性能**：查看执行时间差异

### 自动化测试：

```bash
#!/bin/bash
# 持续集成脚本示例

echo "🔄 Running STARK verification CI..."

# 生成并验证
./crates/nockchain/scripts/quick_start_verification.sh both

# 检查结果
if [ $? -eq 0 ]; then
    echo "✅ CI passed"
    exit 0
else
    echo "❌ CI failed"
    exit 1
fi
```

## 📞 获取帮助

- 查看详细文档：`crates/nockchain/README_STARK_VERIFICATION.md`
- 脚本帮助：`./crates/nockchain/scripts/quick_start_verification.sh help`
- 查看测试列表：`cargo test --test prove_block_fast_test -- --list`

---

**开始使用**：运行 `./crates/nockchain/scripts/quick_start_verification.sh` 即可开始！🚀
