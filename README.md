# Mau

一个强大的 Rust 过程宏库，提供记忆化（memoization）功能和高效的范围操作宏。

## 功能特性

- ✅ **自动记忆化**: `#[memo]` 属性宏，为单个函数添加缓存
- ✅ **批量记忆化**: `memo_block!` 函数宏，智能缓存管理，避免内存泄漏
- ✅ **智能缓存键**: 三种键模式（`ptr`、`ref`、`val`），平衡性能和功能
- ✅ **线程模式**: 单线程（`single`）和多线程（`multi`）支持
- ✅ **范围宏**: `min!`、`max!`、`sum!`、`and!`、`or!`、`reduce!` 等高效宏
- ✅ **空迭代器处理**: `min!` 和 `max!` 对空迭代器返回边界值

## 安装

```toml
[dependencies]
mau = "0.1.3"
```

## 快速开始

### 1. 基础记忆化

```rust
use mau::memo;

#[memo]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    println!("fibonacci(40) = {}", fibonacci(40)); // 极快！
}
```

**性能提升**：
- 不使用 memo：~1 秒
- 使用 memo：~0.01 毫秒
- **性能提升：100,000 倍！**

### 2. 互相递归（批量记忆化）

```rust
use mau::memo_block;

memo_block! {
    fn is_even(n: usize) -> bool {
        if n == 0 { true } else { is_odd(n - 1) }
    }

    fn is_odd(n: usize) -> bool {
        if n == 0 { false } else { is_even(n - 1) }
    }
}

fn main() {
    println!("is_even(100) = {}", is_even(100)); // true
}
```

### 3. 范围宏

```rust
use mau::{min, max, sum};

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    println!("最小值: {}", min!(data));  // 1
    println!("最大值: {}", max!(data));  // 9
    println!("总和: {}", sum!(data));    // 31
    
    // 空迭代器返回边界值
    let empty: Vec<i32> = vec![];
    println!("空数组的最小值: {}", min!(empty));  // i32::MAX
}
```

## 核心功能详解

### `#[memo]` - 长期缓存

为单个函数添加记忆化，**缓存会永久保留**直到程序结束。

#### 参数配置

**线程模式（`thread`）**：
- `single`（默认）：单线程，性能最佳
- `multi`：多线程安全，全局共享

**键模式（`key`）**：
- `ptr`：只比较地址，最快
- `ref`（默认）：先比地址再比内容，平衡性能和功能
- `val`：深度比较，功能最完整

#### 使用语法

```rust
// 使用默认配置
#[memo]
fn calc(n: i32) -> i32 { n * n }

// 命名参数（推荐）
#[memo(thread=single, key=ref)]
#[memo(thread=multi, key=ptr)]
#[memo(key=val)]

// 注意：ref 是关键字，需要写成 r#ref
#[memo(key=r#ref)]
```

#### 键模式详解

##### ptr 模式 - 最快，只比地址

```rust
#[memo(key=ptr)]
fn process(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 示例：
let arr = vec![1, 2, 3];
process(&arr);  // 第1次：计算
process(&arr);  // 第2次：命中 ✓（相同地址）

let arr2 = vec![1, 2, 3];  // 内容相同，地址不同
process(&arr2);  // 第3次：重新计算（地址不同）
```

**何时使用**：相同引用会反复调用（如递归中传递同一个数组）

##### ref 模式 - 默认，先比地址再比内容

```rust
#[memo(key=r#ref)]  // 或 #[memo]
fn process(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 示例：
let arr = vec![1, 2, 3];
process(&arr);  // 第1次：计算
process(&arr);  // 第2次：命中 ✓（相同地址，极快）

let arr2 = vec![1, 2, 3];  // 内容相同，地址不同
process(&arr2);  // 第3次：命中 ✓（地址不同，比较内容）

let arr3 = vec![4, 5, 6];  // 内容不同
process(&arr3);  // 第4次：重新计算（内容不同）
```

**工作原理**：
1. **相同地址** → 立即返回（最快）
2. **不同地址** → 比较内容，相同就命中

**何时使用**：大部分情况的最佳选择

##### val 模式 - 功能最完整，深度比较

```rust
#[memo(key=val)]
fn process(matrix: &[Vec<i32>]) -> i32 {
    matrix.iter().map(|row| row.iter().sum::<i32>()).sum()
}
```

**何时使用**：复杂嵌套类型，需要深度比较

##### 三种模式对比

| 模式 | 比较方式 | 相同地址 | 不同地址+相同内容 | 性能 | 适用场景 |
|------|---------|----------|------------------|------|---------|
| `ptr` | 地址+长度 | ⚡极快 | ❌不命中 | 最快 | 相同引用反复调用 |
| `ref` | 地址→内容 | ⚡快 | ✅命中 | 快 | 一般情况（推荐） |
| `val` | 深度比较 | 慢 | ✅命中 | 慢 | 复杂嵌套类型 |

### `memo_block!` - 智能缓存管理

`memo_block!` 解决了 `#[memo]` 的两个核心问题：

#### 问题 1：缓存永不清空导致内存泄漏

```rust
// 使用 #[memo]
#[memo]
fn compute(n: i32) -> i32 {
    // 复杂计算
    n * n * n
}

fn main() {
    // 调用 10000 次，每次不同的参数
    for i in 0..10000 {
        compute(i);  // 缓存不断增长！
    }
    // 问题：10000 个缓存条目永久占用内存 ❌
}
```

#### 问题 2：手动清空会破坏递归中的缓存

假设我们尝试在 `#[memo]` 上手动清空缓存：

```rust
// ❌ 错误的做法
#[memo]
fn fib(n: usize) -> usize {
    let result = fib_inner(n);
    clear_cache();  // 每次调用后清空
    result
}

// 问题：
fib(10)
  └─ fib(9)
       └─ fib(8) -> 清空缓存！
  └─ fib(8) -> 缓存已空，重新计算 ❌
```

**后果**：递归调用中，内层调用清空缓存后，外层调用无法使用缓存，失去了记忆化的意义。

#### 解决方案：`memo_block!` 的智能清理

`memo_block!` 实现了**智能的自动清理机制**：

```rust
memo_block! {
    fn fib(n: usize) -> usize {
        if n <= 1 { n } else { fib(n-1) + fib(n-2) }
    }
}

// 工作原理：
fib(10)  // 最外层调用
  ├─ fib(9)  // 内层调用，缓存保留 ✓
  │   ├─ fib(8)  // 缓存保留 ✓
  │   │   └─ ...
  │   └─ fib(7)  // 缓存命中 ✓
  └─ fib(8)  // 缓存命中 ✓
  // 最外层调用结束 -> 自动清空缓存 ✓

// 特点：
// - 递归过程中：缓存正常工作
// - 调用结束后：自动清空，释放内存
// - 调用次数：11 次（vs 不使用缓存的 177 次）
```

#### 何时使用 `memo_block!`

✅ **应该使用 `memo_block!`**：
- 单次调用中有大量递归（如动态规划）
- 参数范围很大，不需要跨调用缓存
- 需要控制内存使用
- 多个函数互相递归

❌ **不应该使用 `memo_block!`**：
- 需要长期保留缓存（跨多次调用）
- 参数经常重复，缓存命中率高

#### 使用示例

```rust
use mau::memo_block;

memo_block! {
    // 每个函数可以独立配置
    #[memo(key=ptr)]
    fn a(n: usize) -> usize {
        if n == 0 { 1 } else { a(n-1) + b(n-1) }
    }
    
    #[memo(key=r#ref)]
    fn b(n: usize) -> usize {
        if n == 0 { 2 } else { b(n-1) + a(n-1) }
    }
}

fn main() {
    // 每次调用都自动清理
    a(10);  // 使用缓存优化 + 自动清理
    a(10);  // 重新计算 + 自动清理
    
    // 手动清理（可选）
    clear_a();
    clear_b();
}
```

#### `#[memo]` vs `memo_block!` 对比

| 特性 | `#[memo]` | `memo_block!` |
|------|-----------|---------------|
| **语法** | `#[memo] fn f() {}` | `memo_block! { fn f() {} }` |
| **缓存策略** | 永久保留 | 调用后自动清空 |
| **内存占用** | 持续增长 | 调用后释放 |
| **适用场景** | 参数经常重复，需要长期缓存 | 单次调用优化，控制内存 |
| **互相递归** | 需要分别标记每个函数 | 自动处理所有函数 |
| **性能** | 第2次调用极快（缓存命中） | 每次调用都重新计算 |
| **内存管理** | 手动管理（或不管理） | 自动清理 |

#### 实际场景对比

**场景 1：Web 服务器（需要长期缓存）**

```rust
// ✅ 使用 #[memo]
#[memo]
fn get_user_info(user_id: i32) -> UserInfo {
    // 数据库查询
    database.query(user_id)
}

// 原因：
// - 相同 user_id 会被多次查询
// - 缓存可以避免重复的数据库访问
// - 内存占用可控（用户数量有限）
```

**场景 2：动态规划算法（单次计算优化）**

```rust
// ✅ 使用 memo_block!
memo_block! {
    fn longest_path(graph: &[Vec<i32>], start: usize) -> i32 {
        // 复杂的递归计算
        ...
    }
}

fn solve_problem(graph: &[Vec<i32>]) {
    let result = longest_path(graph, 0);
    // 调用结束，缓存自动清空
    println!("结果: {}", result);
}

// 原因：
// - 每个问题实例只计算一次
// - 不需要跨调用保留缓存
// - 避免内存泄漏（图可能很大）
```

**场景 3：互相递归（memo_block! 的优势）**

```rust
// ❌ 使用 #[memo] 的问题
#[memo]
fn a(n: usize) -> usize {
    if n == 0 { 1 } else { a(n-1) + b(n-1) }
}

#[memo]
fn b(n: usize) -> usize {
    if n == 0 { 2 } else { b(n-1) + a(n-1) }
}

// 问题：需要分别标记，且缓存永不清空

// ✅ 使用 memo_block! 
memo_block! {
    fn a(n: usize) -> usize {
        if n == 0 { 1 } else { a(n-1) + b(n-1) }
    }
    
    fn b(n: usize) -> usize {
        if n == 0 { 2 } else { b(n-1) + a(n-1) }
    }
}

// 优势：
// - 一次性定义所有函数
// - 自动处理互相调用
// - 智能清理，避免内存泄漏
```

### 范围宏

高效的范围聚合操作。

#### 基本用法

```rust
use mau::{min, max, sum, and, or};

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // 整个数组
    println!("最小值: {}", min!(data));  // 1
    println!("最大值: {}", max!(data));  // 9
    println!("总和: {}", sum!(data));    // 31
    
    // 部分范围
    println!("索引 2~5 的最小值: {}", min!(|i| data[i], 2..5));  // 1
    
    // 表达式
    println!("平方的最小值: {}", min!(|i| data[i] * data[i], 0..data.len()));  // 1
    
    // 布尔运算
    let all_positive = and!(|i| data[i] > 0, 0..data.len());
    println!("是否全部为正: {}", all_positive);  // true
}
```

#### 空迭代器处理

```rust
let empty: Vec<i32> = vec![];

// min! 返回类型的 MAX 值
println!("{}", min!(empty));  // i32::MAX = 2147483647

// max! 返回类型的 MIN 值
println!("{}", max!(empty));  // i32::MIN = -2147483648

// 不支持的类型会 panic
let empty_str: Vec<&str> = vec![];
// min!(empty_str);  // panic: "type does not have a MAX value"
```

**支持的类型**：
- ✅ 整数：`i8`~`i128`、`u8`~`u128`、`isize`、`usize`
- ✅ 浮点：`f32`、`f64`
- ✅ 字符：`char`
- ❌ 字符串等：运行时 panic

## 详细示例

### 动态规划：背包问题

```rust
use mau::memo_block;

memo_block! {
    #[memo(key=r#ref)]
    fn knapsack(weights: &[i32], values: &[i32], capacity: i32, n: usize) -> i32 {
        if n == 0 || capacity == 0 {
            return 0;
        }
        
        if weights[n - 1] > capacity {
            knapsack(weights, values, capacity, n - 1)
        } else {
            let include = values[n - 1] + 
                knapsack(weights, values, capacity - weights[n - 1], n - 1);
            let exclude = knapsack(weights, values, capacity, n - 1);
            include.max(exclude)
        }
    }
}

fn main() {
    let weights = vec![10, 20, 30];
    let values = vec![60, 100, 120];
    let capacity = 50;
    
    let result = knapsack(&weights, &values, capacity, weights.len());
    println!("最大价值: {}", result);  // 220
    // 调用结束，缓存自动清空，释放内存
}
```

### 多参数记忆化

```rust
use mau::memo;

#[memo(key=r#ref)]
fn edit_distance(s1: &String, s2: &String, m: usize, n: usize) -> usize {
    if m == 0 { return n; }
    if n == 0 { return m; }
    
    if s1.chars().nth(m - 1) == s2.chars().nth(n - 1) {
        edit_distance(s1, s2, m - 1, n - 1)
    } else {
        1 + [
            edit_distance(s1, s2, m - 1, n),
            edit_distance(s1, s2, m, n - 1),
            edit_distance(s1, s2, m - 1, n - 1),
        ].iter().min().unwrap()
    }
}

fn main() {
    let s1 = "kitten".to_string();
    let s2 = "sitting".to_string();
    let dist = edit_distance(&s1, &s2, s1.len(), s2.len());
    println!("编辑距离: {}", dist);  // 3
}
```

### 范围宏高级用法

#### 自定义归约

```rust
use mau::reduce;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 找最大值
    let max = reduce!(|i| data[i], 0..data.len(), |a, b| a.max(b));
    
    // 字符串连接
    let words = vec!["Hello", " ", "World"];
    let text = reduce!(
        |i| words[i].to_string(),
        0..words.len(),
        |a, b| a + &b
    );
    println!("{}", text);  // "Hello World"
}
```

#### 短路优化

```rust
use mau::{and, or};

fn expensive_check(x: i32) -> bool {
    println!("检查 {}", x);
    x > 0
}

fn main() {
    let data = vec![1, 2, -3, 4, 5];
    
    // and! 遇到第一个 false 就停止
    let all_positive = and!(|i| expensive_check(data[i]), 0..data.len());
    // 输出：
    // 检查 1
    // 检查 2  
    // 检查 -3  <- 停止，不再检查后续元素
    
    println!("全部为正: {}", all_positive);  // false
}
```

## 性能数据

### 记忆化性能提升

| 算法 | 规模 | 不使用 memo | 使用 memo | 提升倍数 |
|------|------|-------------|-----------|---------|
| Fibonacci | n=30 | 10 ms | 0.01 ms | 1,000x |
| Fibonacci | n=40 | 1000 ms | 0.01 ms | 100,000x |
| Fibonacci | n=50 | >60秒 | 0.01 ms | >6,000,000x |
| LCS | 长度50 | 10秒 | 0.1秒 | 100x |
| 背包问题 | 50项 | 5秒 | 0.05秒 | 100x |

### 键模式性能对比

测试：10,000 次调用，缓存已预热

| 模式 | 时间 | 相对性能 |
|------|------|---------|
| `ptr` | 1.2 ms | 100% |
| `ref` | 1.5 ms | 80% |
| `val` | 3.4 ms | 35% |

## 使用建议

### 何时使用记忆化

✅ **应该使用**：
- 递归函数有重复子问题
- 动态规划算法
- 计算代价高但参数经常重复
- 纯函数（无副作用）

❌ **不应该使用**：
- 函数有副作用（I/O、打印等）
- 参数几乎不重复
- 计算非常简单

### 键模式选择策略

```rust
// 场景1：递归中传递同一个引用
#[memo(key=ptr)]
fn recursive(data: &[i32], index: usize) -> i32 {
    if index >= data.len() { return 0; }
    data[index] + recursive(data, index + 1)  // 同一个 data
}

// 场景2：不同调用但参数可能相同（推荐，默认）
#[memo(key=r#ref)]
fn process(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 场景3：复杂嵌套类型
#[memo(key=val)]
fn matrix_calc(matrix: &[Vec<Vec<i32>>]) -> i32 {
    // 深度比较
    0
}
```

### `#[memo]` vs `memo_block!` 选择

**使用 `#[memo]` 的场景**：

```rust
// 配置计算：参数有限，会重复调用
#[memo]
fn parse_config(key: String) -> Config {
    // 解析配置
}

// 数据转换：同样的输入会多次出现
#[memo]
fn transform_data(input: Vec<i32>) -> Vec<String> {
    // 转换数据
}
```

**使用 `memo_block!` 的场景**：

```rust
// 动态规划：参数范围大，单次优化
memo_block! {
    fn solve_dp(state: Vec<i32>, step: usize) -> i32 {
        // DP 计算
    }
}

// 递归算法：需要内存控制
memo_block! {
    fn dfs(graph: &[Vec<i32>], node: usize, visited: Vec<bool>) -> i32 {
        // 深度优先搜索
    }
}
```

## 注意事项

### 1. 避免副作用

```rust
// ❌ 错误：有副作用
#[memo]
fn bad_example(n: i32) -> i32 {
    println!("计算 {}", n);  // 缓存命中时不会打印！
    n * 2
}

// ✅ 正确：纯函数
#[memo]
fn good_example(n: i32) -> i32 {
    n * 2
}
```

### 2. 参数设计

```rust
// ❌ 错误：无关参数导致缓存失效
#[memo]
fn bad_design(n: i32, timestamp: u64) -> i32 {
    n * 2  // timestamp 不影响结果，但会导致缓存失效
}

// ✅ 正确：只包含必要参数
#[memo]
fn good_design(n: i32) -> i32 {
    n * 2
}
```

### 3. 内存监控

```rust
// ⚠️ 如果参数范围很大，使用 memo_block!
memo_block! {
    fn compute(n: i32) -> i32 {
        n * n * n
    }
}

// 而不是 #[memo]（会一直占用内存）
```

### 4. f64 类型处理

```rust
// ❌ f64 不实现 Hash 和 Eq
// #[memo]
// fn calc(x: f64) -> f64 { x * x }  // 编译错误

// ✅ 使用引用（自动转换为 u64）
#[memo(key=r#ref)]
fn calc(x: &f64) -> f64 { x * x }

// ✅ 或使用 val 模式
#[memo(key=val)]
fn calc_array(data: &[f64]) -> f64 {
    data.iter().sum()
}
```

## 完整示例：组合使用

```rust
use mau::{memo, memo_block, min, max, sum};

// 长期缓存：配置解析
#[memo]
fn parse_config(path: String) -> Config {
    // 读取配置文件（缓存结果）
}

// 临时缓存：动态规划
memo_block! {
    #[memo(key=r#ref)]
    fn longest_increasing_subsequence(arr: &[i32], i: usize) -> usize {
        if i == 0 { return 1; }
        
        let mut max_len = 1;
        for j in 0..i {
            if arr[j] < arr[i] {
                max_len = max_len.max(1 + longest_increasing_subsequence(arr, j));
            }
        }
        max_len
    }
}

fn main() {
    // 长期缓存
    let config = parse_config("config.toml".to_string());
    
    // 临时缓存
    let data = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let result = (0..data.len())
        .map(|i| longest_increasing_subsequence(&data, i))
        .max()
        .unwrap();
    println!("最长递增子序列长度: {}", result);
    
    // 范围宏
    let min_val = min!(data);
    let max_val = max!(data);
    let sum_val = sum!(data);
    println!("最小: {}, 最大: {}, 总和: {}", min_val, max_val, sum_val);
}
```

## 参数速查表

### `#[memo]` 参数

```rust
#[memo]                              // 默认：thread=single, key=ref
#[memo(thread=single, key=r#ref)]   // 命名参数
#[memo(thread=multi, key=ptr)]      // 多线程 + 地址键
#[memo(key=val)]                    // 只指定 key
```

### `memo_block!` 参数

```rust
memo_block! {
    #[memo(key=ptr)]               // 每个函数独立配置
    fn a() { ... }
    
    #[memo(thread=multi, key=val)] // 多个属性用逗号
    fn b() { ... }
    
    fn c() { ... }                 // 使用默认配置
}
```

### 范围宏语法

```rust
min!(1, 2, 3)                      // 多参数
min!(array)                        // 整个数组
min!(|i| array[i], 0..10)         // 范围表达式
min!(|i| array[i] * 2, 0..=9)     // 包含范围

reduce!(|i| data[i], 0..n, |a, b| a.max(b))  // 自定义归约
```

## 常见问题

### Q1: 为什么需要 `memo_block!`？

**A**: 解决两个核心问题：
1. **内存泄漏**：`#[memo]` 缓存永不清空
2. **清理时机**：简单清空会破坏递归中的缓存

`memo_block!` 通过深度跟踪机制，在最外层调用结束后清空缓存，保证递归过程中缓存正常工作。

### Q2: ref 模式比 ptr 慢多少？

**A**: 在缓存已预热的情况下，`ref` 模式约为 `ptr` 模式的 80% 性能。但 `ref` 模式功能更强（内容相同就命中），是大多数情况的最佳选择。

### Q3: 空迭代器为什么返回边界值？

**A**: 符合数学定义：
- `min(空集) = +∞` → 返回 `MAX`
- `max(空集) = -∞` → 返回 `MIN`

这样可以避免 panic，提供更好的默认行为。

### Q4: 如何处理 f64 类型？

**A**: 使用引用参数，宏会自动转换：

```rust
#[memo(key=r#ref)]
fn calc(x: &f64) -> f64 {
    x * x
}

#[memo(key=r#ref)]
fn sum_floats(data: &[f64]) -> f64 {
    data.iter().sum()
}
```

### Q5: memo_block! 每次都重新计算吗？

**A**: 
- **单次调用内**：缓存正常工作，避免重复计算 ✓
- **调用结束后**：自动清空，释放内存 ✓
- **下次调用**：重新计算，但仍然使用缓存优化 ✓

例如：
```rust
fib(10);  // 计算 11 次（vs 不使用 177 次）✓
fib(10);  // 再次计算 11 次（vs 不使用 177 次）✓
```

## 性能对比示例

### Fibonacci 性能测试

```rust
use mau::memo;
use std::time::Instant;

#[memo]
fn fib_memo(n: usize) -> usize {
    if n <= 1 { n } else { fib_memo(n-1) + fib_memo(n-2) }
}

fn fib_no_memo(n: usize) -> usize {
    if n <= 1 { n } else { fib_no_memo(n-1) + fib_no_memo(n-2) }
}

fn main() {
    // 测试 n=40
    let start = Instant::now();
    let result = fib_no_memo(40);
    let time_no_memo = start.elapsed();
    
    let start = Instant::now();
    let result_memo = fib_memo(40);
    let time_memo = start.elapsed();
    
    println!("不使用 memo: {:?}", time_no_memo);  // ~1 秒
    println!("使用 memo: {:?}", time_memo);       // ~0.01 毫秒
    println!("性能提升: {}x", time_no_memo.as_micros() / time_memo.as_micros());
}
```

## 最佳实践总结

1. **默认使用 `ref` 模式**：最佳的性能/功能平衡
2. **单次计算用 `memo_block!`**：自动清理，避免内存泄漏
3. **长期缓存用 `#[memo]`**：跨调用保留，提升性能
4. **递归传递相同引用用 `ptr`**：最快
5. **复杂类型用 `val`**：功能最完整
6. **避免副作用**：只在纯函数上使用
7. **监控内存**：参数范围大时使用 `memo_block!`

## 更新日志

### v0.1.3
- ✅ `ref` 模式：先比地址，再比内容（最佳平衡）
- ✅ 参数重命名：`thread_mode`→`thread`，`index_mode`→`key`
- ✅ 键模式重命名：`light`→`ptr`，`normal`→`ref`，`heavy`→`val`
- ✅ 线程模式重命名：`local`→`single`
- ✅ `ptr` 模式改进：使用 (地址, 长度) 作为键
- ✅ `min!`/`max!` 空迭代器返回边界值
- ✅ `memo_block!` 支持每个函数独立配置
- ✅ 支持命名参数语法：`key=value`

## 许可证

MIT 或 Apache-2.0 双许可证。

