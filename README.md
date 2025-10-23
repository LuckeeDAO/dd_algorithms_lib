# 去中心化决策算法库

一个用于去中心化决策、公平分配算法和随机数生成的库。该库专注于共识友好、客观的算法。

## 功能特性

### ⚖️ 公平分配算法
- 等权重和加权公平分配
- 超级公平分配算法
- 最优资源配置

### 🎲 去中心化随机数生成
- 使用异或运算的单数和多数随机数生成
- 当输入大小为2的n次幂时，保证概率分布相等
- 防冲突和唯一性保证
- 支持白名单排除选择

## 安装

将以下内容添加到你的 `Cargo.toml` 中：

```toml
[dependencies]
dd_algorithms_lib = "0.1.0"
```

### 基本用法

```rust
use dd_algorithms_lib::{
    // 公平分配
    calculate_fair_division_equal_weights,
    calculate_fair_division_weighted,
    
    // 随机数生成
    get_one_dd_rand_num,
    get_one_dd_3d_rand_num,
    get_k_dd_rand_num,
    get_k_dd_rand_num_with_whitelist,
};

// 公平分配示例
let bids = [100i128, 200, 300];
let mut allocation = [0i128; 3];
calculate_fair_division_equal_weights(&bids, &mut allocation).unwrap();
// allocation: [66, 133, -199] (sum = 0)

// 单个随机数生成（基于异或运算）
let values = [100u128, 200, 300, 400]; // 4个值 (2^2)
let n = values.len();
let mut result = 0u128;
get_one_dd_rand_num(&values, n, &mut result).unwrap();
// result = 100 ^ 200 ^ 300 ^ 400

// 多个随机选择（基于异或运算）
let group1 = [100u128, 200, 300];
let group2 = [150u128, 250, 350];
let group3 = [120u128, 220, 320];
let group4 = [130u128, 230, 330]; // 4个组 (2^2)
let groups = [group1.as_slice(), group2.as_slice(), group3.as_slice(), group4.as_slice()];
let mut selected = [0usize; 3];
get_k_dd_rand_num(&groups, 4, 3, &mut selected).unwrap();
// selected: [1, 2, 0] (唯一的参与者索引)
```

## 模块

### `algorithms`
用于公平分配和随机数生成的数学算法：
- `calculate_fair_division_equal_weights()` - 等权重公平分配
- `calculate_fair_division_weighted()` - 自定义权重公平分配
- `get_one_dd_rand_num()` - 生成单个去中心化随机数（基于异或运算）
- `get_one_dd_3d_rand_num()` - 生成单个彩票随机数（基于异或运算）
- `get_k_dd_rand_num()` - 生成多个唯一随机数（基于异或运算）
- `get_k_dd_rand_num_with_whitelist()` - 生成带排除的多个随机数（基于异或运算）

### `types`
通用数据类型和枚举：
- `VotingPower` - 投票权重类型别名
- `ParticipantId` - 参与者标识符类型
- `Timestamp` - 时间戳类型别名
- `FairDivisionResult` - 公平分配结果结构（如果使用）
- `RandomSelectionResult` - 随机选择结果结构（如果使用）

## 算法详情

### 公平分配
实现超级公平分配算法，确保：
- 零和分配（所有分配的总和等于零）
- 基于输入值或权重的公平分配
- 最优资源配置

### 基于异或运算的随机数生成
该库使用异或（XOR）运算进行随机数生成，具有以下优势：

#### 为什么选择异或运算？
- **概率相等**：当输入大小为2的n次幂时，异或运算确保每个可能结果具有相等概率
- **高效性**：异或运算比模运算更快
- **交换律**：输入顺序不影响结果 (A ⊕ B = B ⊕ A)
- **结合律**：分组不影响结果 ((A ⊕ B) ⊕ C = A ⊕ (B ⊕ C))
- **确定性**：相同输入总是产生相同输出（对共识很重要）

#### 数学性质
- 异或是自身的逆运算：A ⊕ A = 0
- 与零异或是恒等运算：A ⊕ 0 = A
- 异或满足分配律：A ⊕ (B ⊕ C) = (A ⊕ B) ⊕ C

### 随机数生成
使用异或运算的去中心化随机数生成，具有以下特性：
- **基于异或运算的算法**：使用位异或运算实现概率分布相等
- **2的n次幂约束**：输入大小必须是2的n次幂以获得最优随机性
- **防冲突**：确保所有生成的数字都是唯一的
- **不可预测性**：使用参与者提供的随机值
- **偏移机制**：通过系统偏移防止模式
- **白名单支持**：从选择中排除特定参与者
- **验证**：全面的参数验证和边界检查

## 示例

查看 `examples/` 目录获取全面的使用示例：
- `governance_example.rs` - 所有功能的完整演示
- `function_names_test.rs` - 函数名称验证和测试

## 约束条件

- **参与者数量 (n)**：≤ 100,000 且必须是2的n次幂 (2^n)
- **选择数量 (k)**：≤ 1,000
- **k ≤ n**：不能选择比可用参与者更多的参与者
- **异或算法**：需要2的n次幂输入大小以实现概率分布相等

## 许可证

本项目采用 MIT 许可证 - 详情请查看 LICENSE 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 发布到 crates.io

按照以下检查清单将此库的新版本发布到 crates.io。

### 1) 前置条件
- 创建并验证 crates.io 账户
- （推荐）在 crates.io 上启用 2FA
- 在 crates.io → Account → API Tokens 创建 API 令牌
- 本地登录：

```bash
cargo login <YOUR_API_TOKEN>
```

### 2) 验证 Cargo.toml 元数据
确保以下字段正确：`name`、`version`、`description`、`license`、`repository`、`documentation`、`readme`、`keywords`、`categories`、`rust-version`。此库默认为 `no_std` 并提供可选的 `std`、`serde` 和 `log_tests` 功能。

### 3) 构建、测试、文档

```bash
cargo clean
cargo test
# 可选：显示测试日志
cargo test --features log_tests -- --nocapture
# 本地文档
cargo doc --no-deps
```

### 4) 打包和试运行

```bash
cargo package
cargo publish --dry-run
```

### 5) 发布

```bash
cargo publish
```

### 6) 版本控制和标签

```bash
# 首先在 Cargo.toml 中提升版本
git tag -a v0.1.0 -m "Release v0.1.0"
git push --tags
```

### 7) 管理所有者

```bash
cargo owner --add <github-user-or-team>
cargo owner --list
```

### 8) 故障排除
- 如果打包内容错误，使用 `cargo package` 检查并调整 `Cargo.toml` 或 `.gitignore` 中的 `include`/`exclude`。
- 对于 docs.rs 上的 `no_std` 文档，确保默认功能不会引入 `std`。此库默认使用 `#![no_std]`。
- 测试中的打印功能被限制在可选的 `log_tests` 功能后面，以保持默认的 `no_std` 行为。

### 9) 发布修复
Crates 无法被覆盖。提升版本并重新发布。你可以撤回错误的版本：

```bash
cargo yank --vers <version>
# 如有必要，撤销
cargo yank --vers <version> --undo
```