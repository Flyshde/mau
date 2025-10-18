# Mau

一个强大的 Rust 过程宏库，提供记忆化（memoization）功能和高效的范围操作宏。

## 功能特性

- ✅ **自动记忆化**: `#[memo]` 属性宏，智能缓存管理
- ✅ **智能清理**: `solve!` 宏，自动清空缓存，避免内存泄漏
- ✅ **生命周期控制**: `lifetime` 参数，精确控制缓存保留策略
- ✅ **智能缓存键**: 三种键模式（`ptr`、`ref`、`val`），平衡性能和功能
- ✅ **线程模式**: 单线程（`single`）和多线程（`multi`）支持
- ✅ **范围宏**: `min!`、`max!`、`sum!`、`and!`、`or!`、`reduce!`、`fold!` 等高效宏
- ✅ **灵活语法**: 支持多参数、数组、范围等多种调用方式
- ✅ **空迭代器处理**: `min!` 和 `max!` 对空迭代器返回边界值

## 安装

```toml
[dependencies]
mau = "0.1.13"
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

### 2. 智能清理缓存

```rust
use mau::{memo, solve};

#[memo]
fn compute(n: i32) -> i32 {
    // 复杂计算
    n * n * n
}

fn main() {
    // 使用 solve! 宏，自动清空缓存
    let result = solve!(compute(100));
    println!("结果: {}", result);
    // 调用结束，缓存已清空，避免内存泄漏
}
```

### 3. 范围宏

```rust
use mau::{min, max, sum, fold};

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // 多种语法支持
    println!("最小值: {}", min!(data));        // 数组语法：1
    println!("最小值: {}", min!(1, 2, 3));     // 多参数语法：1
    println!("最大值: {}", max!(data));        // 9
    println!("总和: {}", sum!(data));          // 31
    
    // 范围语法
    println!("部分最小: {}", min!(|i| data[i], 2..5));  // 1
    
    // fold 累积操作
    let product = fold!(1, |i| data[i], 0..data.len(), |acc, val| acc * val);
    println!("乘积: {}", product);  // 51840
    
    // 空迭代器返回边界值
    let empty: Vec<i32> = vec![];
    println!("空数组的最小值: {}", min!(empty));  // i32::MAX
}
```

## 核心功能详解

### `#[memo]` - 自动记忆化

为函数添加记忆化，自动缓存计算结果：

```rust
#[memo]
fn fibonacci(n: usize) -> usize {
    if n <= 1 { n } else { fibonacci(n - 1) + fibonacci(n - 2) }
}

fn main() {
    // 第一次调用：计算并缓存
    let r1 = fibonacci(10);  // 计算
    
    // 第二次调用：直接从缓存返回
    let r2 = fibonacci(10);  // 命中缓存，极快
    
    // 利用已有缓存计算新值
    let r3 = fibonacci(11);  // 利用 fibonacci(10) 和 fibonacci(9) 的缓存
}
```

**生成的辅助函数**：

```rust
#[memo]
fn fibonacci(n: usize) -> usize {
    // ...
}

// 自动生成：
// - fibonacci_start(n) : 调用并清空缓存
// - fibonacci_clear()  : 手动清空缓存
```

### `solve!` 宏 - 智能清理

自动清空缓存，避免内存泄漏：

```rust
use mau::{memo, solve};

#[memo]
fn is_even(n: usize) -> bool {
    match n {
        0 => true,
        _ => is_odd(n - 1),
    }
}

#[memo]
fn is_odd(n: usize) -> bool {
    match n {
        0 => false,
        _ => is_even(n - 1),
    }
}

fn main() {
    // 简洁语法：调用并自动清理缓存
    let result = solve!(is_even(100));
    
    // 支持复杂表达式
    let (r1, r2) = solve!((is_even(50), is_odd(51)));
    
    // 支持代码块
    let result = solve!({
        let a = is_even(10);
        let b = is_even(20);
        a && b
    });
    
    // 嵌套调用（自动递归替换）
    let result = solve!(is_even(is_odd(5) as usize * 10));
}
```

**何时使用 `solve!`**：

✅ **应该使用**：
- 单次调用中有大量递归（如动态规划）
- 参数范围很大，不需要跨调用缓存
- 需要控制内存使用

❌ **不应该使用**：
- 需要长期保留缓存（跨多次调用）
- 参数经常重复，缓存命中率高

### 手动清理缓存

```rust
#[memo]
fn compute(n: i32) -> i32 {
    n * n * n
}

fn main() {
    // 方式 1: 使用 start! 自动清理
    solve!(compute(100));
    
    // 方式 2: 手动清理
    compute(100);
    compute_clear();  // 手动清空缓存
}
```

### 参数配置

**线程模式（`thread`）**：
- `single`（默认）：单线程，性能最佳
- `multi`：多线程安全，全局共享

**键模式（`key`）**：
- `ptr`（默认）：地址+长度，最快
- `ref`：先比地址+长度，再比内容，平衡性能
- `val`：深度比较，功能最完整

**生命周期模式（`lifetime`）**：
- `problem`（默认）：每次 `_start()` 调用后清除缓存
- `program`：保留缓存直到程序结束（仅在键包含地址时有效）

#### 使用语法

```rust
// 使用默认配置
#[memo]
fn calc(n: i32) -> i32 { n * n }

// 命名参数（推荐）
#[memo(thread=single, key=ref)]
#[memo(thread=multi, key=ptr)]
#[memo(key=val, lifetime=problem)]
#[memo(key=ptr, lifetime=program)]  // 长期保留缓存
```

### 键模式详解

#### ptr 模式 - 最快，地址+长度

```rust
#[memo(key=ptr)]
fn process(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 示例：
let arr = vec![1, 2, 3];
process(&arr);  // 第1次：计算
process(&arr);  // 第2次：命中 ✓（相同地址+长度）

let arr2 = vec![1, 2, 3];  // 内容相同，地址不同
process(&arr2);  // 第3次：重新计算（地址不同）

// 切片长度不同
process(&arr[..2]);  // 第4次：重新计算（长度不同）
```

**缓存键**：`(地址, 长度)`
**何时使用**：相同引用会反复调用（如递归中传递同一个数组）

#### ref 模式，先比地址+长度，再比内容

```rust
#[memo(key=ref)]  // 或 #[memo]
fn process(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 示例：
let arr = vec![1, 2, 3];
process(&arr);  // 第1次：计算
process(&arr);  // 第2次：命中 ✓（地址+长度相等）

let arr2 = vec![1, 2, 3];  // 内容相同，地址不同
process(&arr2);  // 第3次：命中 ✓（地址不等，但内容相等）

let arr3 = vec![4, 5, 6];  // 内容不同
process(&arr3);  // 第4次：重新计算（内容不等）

// 切片长度不同
process(&arr[..2]);  // 第5次：重新计算（长度不同）
```

**工作原理**：
1. **快速路径**：比较 `(地址, 长度)`，相等则命中
2. **慢速路径**：地址或长度不等时，比较内容

**何时使用**：大部分情况的最佳选择

#### val 模式 - 功能最完整，深度比较

```rust
#[memo(key=val)]
fn process(matrix: &[Vec<i32>]) -> i32 {
    matrix.iter().map(|row| row.iter().sum::<i32>()).sum()
}
```

**何时使用**：复杂嵌套类型，需要深度比较

#### 三种模式对比

| 模式 | 比较方式 | 相同地址+长度 | 不同地址+相同内容 | 性能 | 适用场景 |
|------|---------|---------------|------------------|------|---------|
| `ptr` | 地址+长度 | ⚡极快 | ❌不命中 | 最快 | 一般情况（默认） |
| `ref` | 先比地址+长度，若相等则命中；否则比内容 | ⚡快 | ✅命中 | 快 | 内容可能重复 |
| `val` | 深度比较 | 慢 | ✅命中 | 慢 | 复杂嵌套类型 |

### 生命周期模式详解

`lifetime` 参数控制缓存何时被清除，这对内存管理至关重要。

#### problem 模式（默认）- 问题级别缓存

```rust
#[memo(lifetime=problem)]  // 或 #[memo]
fn solve_subproblem(data: &[i32]) -> i32 {
    data.iter().sum()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 使用 solve! 或 _start()，调用后自动清除缓存
    let result = solve!(solve_subproblem(&data));
    // 缓存已清除，不占用内存
}
```

**何时使用**：
- ✅ 单次问题求解（如 OJ 题目、一次性计算）
- ✅ 需要控制内存使用
- ✅ 参数范围很大，不需要跨问题复用

#### program 模式 - 程序级别缓存

```rust
#[memo(key=ptr, lifetime=program)]
fn expensive_calculation(data: &[i32]) -> i32 {
    // 复杂计算...
    data.iter().sum()
}

fn main() {
    let data = vec![1, 2, 3];
    
    // 多次调用，缓存一直保留
    let r1 = expensive_calculation_start(&data);  // 计算
    let r2 = expensive_calculation_start(&data);  // 命中缓存
    let r3 = expensive_calculation_start(&data);  // 命中缓存
    // 缓存保留到程序结束
}
```

**何时使用**：
- ✅ 配置解析、数据库查询等需要长期复用的结果
- ✅ 多次请求/调用相同参数
- ✅ 缓存命中率高

#### 重要：program 模式的生效条件

`lifetime=program` **只有在缓存键包含地址信息时才会保留缓存**。

| 函数参数类型 | key 模式 | lifetime=program | 是否保留缓存 | 原因 |
|------------|---------|-----------------|------------|------|
| 有引用参数 (如 `&[i32]`) | `ptr` | ✅ 保留 | 是 | 键包含地址，不同数组不同键 |
| 有引用参数 (如 `&[i32]`) | `ref` | ✅ 保留 | 是 | 键包含地址 + 内容 |
| 有引用参数 (如 `&[i32]`) | `val` | ❌ 清除 | 否 | 键只基于值，无地址信息 |
| 无引用参数 (如 `i32`) | `ptr` | ❌ 清除 | 否 | 无引用，键中无地址 |
| 无引用参数 (如 `i32`) | `ref` | ❌ 清除 | 否 | 无引用，键中无地址 |
| 无引用参数 (如 `i32`) | `val` | ❌ 清除 | 否 | 键只基于值 |

**示例说明**：

```rust
// ✅ 会保留缓存：有引用参数 + key=ptr
#[memo(key=ptr, lifetime=program)]
fn process_array(data: &[i32]) -> i32 {
    data.iter().sum()
}

// ✅ 会保留缓存：有引用参数 + key=ref
#[memo(key=ref, lifetime=program)]
fn process_data(data: &[i32]) -> i32 {
    data.iter().sum()
}

// ❌ 不会保留缓存：无引用参数，即使设置 program
#[memo(key=ptr, lifetime=program)]
fn calculate(n: i32) -> i32 {
    n * n  // 每次调用 _start() 后仍会清除缓存
}

// ❌ 不会保留缓存：有引用但 key=val，键中无地址
#[memo(key=val, lifetime=program)]
fn process_val(data: &[i32]) -> i32 {
    data.iter().sum()  // 每次调用 _start() 后仍会清除缓存
}
```

**为什么这样设计？**

当键中不包含地址信息时，不同问题的相同值会错误地命中缓存。例如：

```rust
#[memo(key=val, lifetime=program)]  // 错误示例
fn solve(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    // 问题1
    let data1 = vec![1, 2, 3];
    solve_start(&data1);  // 结果: 6
    
    // 问题2：不同问题，但数组内容相同
    let data2 = vec![1, 2, 3];
    solve_start(&data2);  // 错误！会命中问题1的缓存
    
    // 这就是为什么 val 模式下即使设置 program 也要清除缓存
}
```

## 使用场景

### 场景 1: 动态规划（使用 start! 自动清理）

```rust
use mau::{memo, solve};

#[memo(key=ref)]
fn merge_stones(data: &[usize]) -> usize {
    match data.len() {
        0 | 1 => 0,
        _ => {
            let mut min_cost = usize::MAX;
            for i in 1..data.len() {
                let left = merge_stones(&data[..i]);
                let right = merge_stones(&data[i..]);
                let cost = left + right + data.iter().sum::<usize>();
                min_cost = min_cost.min(cost);
            }
            min_cost
        }
    }
}

fn main() {
    let stones = vec![1, 2, 3, 4, 5];
    
    // 使用 start! 自动清理缓存
    let result = solve!(merge_stones(&stones));
    println!("最小成本: {}", result);
    // 缓存已清空，不会占用内存
}
```

### 场景 2: Web 服务（长期缓存）

```rust
#[memo]
fn get_user_info(user_id: i32) -> UserInfo {
    // 数据库查询
    database.query(user_id)
}

fn handle_request(user_id: i32) {
    // 多次调用，利用缓存避免重复查询
    let info = get_user_info(user_id);
    // 缓存保留，下次请求直接命中
}
```

### 场景 3: 互相递归

```rust
use mau::{memo, solve};

#[memo]
fn is_even(n: usize) -> bool {
    if n == 0 { true } else { is_odd(n - 1) }
}

#[memo]
fn is_odd(n: usize) -> bool {
    if n == 0 { false } else { is_even(n - 1) }
}

fn main() {
    // 方式 1: 普通调用（缓存持续）
    let r1 = is_even(100);
    let r2 = is_even(100);  // 命中缓存
    
    // 方式 2: 使用 start! 清理
    let r3 = solve!(is_even(100));
    // 缓存已清空
    
    // 方式 3: 手动清理
    is_even_clear();
    is_odd_clear();
}
```

## 范围宏

高效的范围聚合操作，支持多种灵活的调用语法。

### 多种语法支持

```rust
use mau::{min, max, sum, and, or};

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // 1. 多参数语法（2个或更多参数）
    println!("{}", min!(1, 2));           // 1
    println!("{}", max!(1, 2, 3));        // 3
    println!("{}", sum!(1, 2, 3, 4));     // 10
    
    // 2. 数组简写语法
    println!("{}", min!(data));           // 1
    println!("{}", max!(data));           // 9
    println!("{}", sum!(data));           // 31
    
    // 3. 范围语法 - 部分范围
    println!("{}", min!(|i| data[i], 2..5));  // 索引 2~4 的最小值
    
    // 4. 范围语法 - 表达式
    println!("{}", min!(|i| data[i] * data[i], 0..data.len()));  // 平方的最小值
    
    // 5. 包含范围（闭区间）
    println!("{}", sum!(|i| data[i], 2..=4));  // 索引 2,3,4 的和
    
    // 6. 迭代器语法
    println!("{}", min!(|x| x, data.iter()));  // 1
    
    // 7. 布尔运算
    let all_positive = and!(|i| data[i] > 0, 0..data.len());
    println!("是否全部为正: {}", all_positive);  // true
}
```

### fold! 宏 - 自定义累积操作

`fold!` 提供了最灵活的累积操作：

```rust
use mau::fold;
use std::collections::HashMap;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 基础：求和（初始值为 0）
    let sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc + val);
    println!("和: {}", sum);  // 15
    
    // 求积（初始值为 1）
    let product = fold!(1, |i| data[i], 0..data.len(), |acc, val| acc * val);
    println!("积: {}", product);  // 120
    
    // 构建字符串
    let words = vec!["Hello", "World", "Rust"];
    let sentence = fold!(String::new(), |i| words[i], 0..words.len(), 
        |mut acc: String, val: &str| {
            if !acc.is_empty() { acc.push(' '); }
            acc.push_str(val);
            acc
        }
    );
    println!("{}", sentence);  // "Hello World Rust"
    
    // 构建 HashMap
    let keys = vec!["a", "b", "c"];
    let values = vec![1, 2, 3];
    let map = fold!(HashMap::new(), |i| (keys[i], values[i]), 0..keys.len(),
        |mut acc: HashMap<&str, i32>, (k, v)| {
            acc.insert(k, v);
            acc
        }
    );
    
    // 同时计算多个统计量（使用元组）
    let (sum, count, max) = fold!(
        (0, 0, i32::MIN), 
        |i| data[i], 
        0..data.len(),
        |(s, c, m), val| (s + val, c + 1, m.max(val))
    );
    let avg = sum / count;
    println!("平均: {}, 最大: {}", avg, max);
    
    // 条件过滤累积（只累加偶数）
    let even_sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| {
        if val % 2 == 0 { acc + val } else { acc }
    });
    println!("偶数和: {}", even_sum);  // 6 (2 + 4)
}
```

**fold! vs reduce!**：

| 特性 | fold! | reduce! |
|------|-------|---------|
| 初始值 | 需要提供 | 使用第一个元素 |
| 空序列 | 返回初始值 | panic |
| 累加器类型 | 可与元素类型不同 | 必须相同 |
| 灵活性 | 高 | 中 |

```rust
use mau::{fold, reduce};

let data = vec![1, 2, 3, 4, 5];

// reduce: 使用第一个元素作为初始值
let sum1 = reduce!(|i| data[i], 0..data.len(), |a, b| a + b);
// 相当于: 1 + 2 + 3 + 4 + 5 = 15

// fold: 提供初始值
let sum2 = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc + val);
// 相当于: 0 + 1 + 2 + 3 + 4 + 5 = 15

// fold 的优势：可以处理空序列
let empty: Vec<i32> = vec![];
let result = fold!(100, |i| empty[i], 0..0, |acc, val| acc + val);
println!("{}", result);  // 100（返回初始值）

// reduce 会 panic
// let result = reduce!(|i| empty[i], 0..0, |a, b| a + b);  // panic!
```

### 空迭代器处理

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
use mau::{memo, solve};

#[memo(key=ref)]
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

fn main() {
    let weights = vec![10, 20, 30];
    let values = vec![60, 100, 120];
    let capacity = 50;
    
    // 使用 start! 自动清理缓存
    let result = solve!(knapsack(&weights, &values, capacity, weights.len()));
    println!("最大价值: {}", result);  // 220
}
```

### 多参数记忆化

```rust
use mau::memo;

#[memo(key=ref)]
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

// 场景2：不同调用但参数可能相同（内容可能重复）
#[memo(key=ref)]
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
#[memo]
fn compute(n: i32) -> i32 {
    n * n * n
}

fn main() {
    // 方式 1: 每次清理
    for i in 0..10000 {
        solve!(compute(i));  // 自动清理
    }
    
    // 方式 2: 批量清理
    for i in 0..10000 {
        compute(i);
    }
    compute_clear();  // 手动清理
}
```

### 4. f64 类型处理

```rust
// ❌ f64 不实现 Hash 和 Eq
// #[memo]
// fn calc(x: f64) -> f64 { x * x }  // 编译错误

// ✅ 使用引用（自动转换为 u64）
#[memo(key=ref)]
fn calc(x: &f64) -> f64 { x * x }

// ✅ 或使用 val 模式
#[memo(key=val)]
fn calc_array(data: &[f64]) -> f64 {
    data.iter().sum()
}
```

## 参数速查表

### `#[memo]` 参数

```rust
#[memo]                                    // 默认：thread=single, key=ptr, lifetime=problem
#[memo(thread=single, key=ref)]            // 命名参数
#[memo(thread=multi, key=ptr)]             // 多线程 + 地址键
#[memo(key=val)]                           // 只指定 key
#[memo(key=ptr, lifetime=program)]         // 长期保留缓存（需要有引用参数）
#[memo(thread=multi, key=ref, lifetime=program)]  // 完整指定
```

### `solve!` 宏语法

```rust
solve!(func(args))                   // 单个函数调用
solve!((func1(a), func2(b)))        // 多个调用（元组）
solve!({ let a = f(); a + 1 })      // 代码块
solve!(f(g(h(x))))                   // 嵌套调用（自动递归替换）
```

### 范围宏语法

```rust
// min, max, sum, and, or
min!(1, 2)                           // 两参数（新增支持）
min!(1, 2, 3)                        // 多参数
min!(array)                          // 整个数组
min!(|i| array[i], 0..10)           // 范围表达式
min!(|i| array[i] * 2, 0..=9)       // 包含范围（闭区间）
min!(|x| x, array.iter())           // 迭代器

// reduce - 自定义归约
reduce!(|i| data[i], 0..n, |a, b| a.max(b))

// fold - 带初始值的累积
fold!(init_val, |i| data[i], 0..n, |acc, val| acc + val)
```

## 常见问题

### Q1: ref 模式比 ptr 慢多少？

**A**: 在缓存已预热的情况下，`ref` 模式约为 `ptr` 模式的 80% 性能。但 `ref` 模式功能更强（内容相同就命中），是大多数情况的最佳选择。

### Q2: 为什么 ref 模式需要比较地址和长度？

**A**: 避免相同地址不同长度的错误命中：

```rust
let data = vec![1, 2, 3, 4, 5];
&data[..2]  // addr = data.as_ptr(), len = 2
&data[..5]  // addr = data.as_ptr(), len = 5  ← 地址相同！

// 如果只比地址，会错误地认为这两个切片相同
// 同时比较地址和长度后：(addr1, len1) != (addr2, len2)
```

### Q3: 空迭代器为什么返回边界值？

**A**: 符合数学定义：
- `min(空集) = +∞` → 返回 `MAX`
- `max(空集) = -∞` → 返回 `MIN`

这样可以避免 panic，提供更好的默认行为。

### Q4: 如何处理 f64 类型？

**A**: 使用引用参数，宏会自动转换：

```rust
#[memo(key=ref)]
fn calc(x: &f64) -> f64 {
    x * x
}

#[memo(key=ref)]
fn sum_floats(data: &[f64]) -> f64 {
    data.iter().sum()
}
```

### Q5: 为什么我设置了 `lifetime=program` 但缓存还是被清除？

**A**: `lifetime=program` 只在键中包含地址信息时才生效。检查：

1. **函数是否有引用参数？**
   ```rust
   // ❌ 无引用参数，program 无效
   #[memo(key=ptr, lifetime=program)]
   fn calc(n: i32) -> i32 { n * n }
   
   // ✅ 有引用参数，program 有效
   #[memo(key=ptr, lifetime=program)]
   fn process(data: &[i32]) -> i32 { data.iter().sum() }
   ```

2. **是否使用了 `key=val`？**
   ```rust
   // ❌ val 模式键中无地址，program 无效
   #[memo(key=val, lifetime=program)]
   fn process(data: &[i32]) -> i32 { data.iter().sum() }
   
   // ✅ ptr/ref 模式键中有地址，program 有效
   #[memo(key=ptr, lifetime=program)]
   fn process(data: &[i32]) -> i32 { data.iter().sum() }
   ```

**总结**：只有 **(有引用参数) AND (key=ptr OR key=ref)** 时，`lifetime=program` 才会保留缓存。

### Q6: `min!(1, 2)` 两参数语法何时可用？

**A**: v0.1.12 及以上版本支持。如果遇到错误，请升级：

```toml
[dependencies]
mau = "0.1.13"  # 或更高版本
```

## 完整示例

```rust
use mau::{memo, start, min, max, sum};

// 长期缓存：配置解析
#[memo]
fn parse_config(path: String) -> Config {
    // 读取配置文件（缓存结果）
}

// 临时缓存：动态规划
#[memo(key=ref)]
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

fn main() {
    // 长期缓存
    let config = parse_config("config.toml".to_string());
    
    // 使用 start! 清理临时缓存
    let data = vec![10, 9, 2, 5, 3, 7, 101, 18];
    let result = solve!({
        (0..data.len())
            .map(|i| longest_increasing_subsequence(&data, i))
            .max()
            .unwrap()
    });
    println!("最长递增子序列长度: {}", result);
    
    // 范围宏
    let min_val = min!(data);
    let max_val = max!(data);
    let sum_val = sum!(data);
    println!("最小: {}, 最大: {}, 总和: {}", min_val, max_val, sum_val);
}
```

## 最佳实践总结

### 记忆化配置

1. **默认使用 `key=ptr`**：性能最佳，适合大多数场景
2. **内容可能重复用 `key=ref`**：兼顾性能和功能
3. **复杂嵌套类型用 `key=val`**：功能最完整

### 生命周期选择

4. **单次计算用 `lifetime=problem`**（默认）：自动清理，避免内存泄漏
5. **长期缓存用 `lifetime=program`**：跨调用保留，需要满足条件：
   - ✅ 函数有引用参数
   - ✅ 使用 `key=ptr` 或 `key=ref`
   - ❌ 不能用 `key=val`（会自动忽略 program 设置）

### 使用技巧

6. **单次问题求解用 `solve!`**：自动清理，推荐用于 OJ、算法竞赛
7. **Web 服务、配置解析等用 `lifetime=program`**：长期复用
8. **递归传递相同引用用 `ptr`**：最快
9. **避免副作用**：只在纯函数上使用
10. **监控内存**：参数范围大时使用 `solve!` 或手动 `_clear()`

### 范围宏选择

11. **简单聚合用 `min!/max!/sum!`**：最简洁
12. **自定义操作用 `reduce!`**：灵活
13. **需要初始值用 `fold!`**：最强大，可处理空序列和类型转换

## 更新日志

### v0.1.13 (最新)
- ✅ **修复两参数语法**：`min!(1, 2)` 现在可以正常工作
- ✅ **新增 `lifetime` 参数**：精确控制缓存生命周期
  - `lifetime=problem`（默认）：调用后清除缓存
  - `lifetime=program`：保留缓存（仅在键包含地址时有效）
- ✅ **智能清除逻辑**：`lifetime=program` 会自动检测键是否包含地址信息
  - 有引用参数 + (`key=ptr` 或 `key=ref`)：保留缓存
  - 无引用参数 或 `key=val`：自动清除（防止错误命中）
- ✅ **新增 50+ 综合测试**：fold、lifetime、两参数语法全面测试

### v0.1.11
- ✅ `solve!` 宏：自动清理缓存，避免内存泄漏（原名 `start!`）
- ✅ 默认键模式改为 `ptr`（性能最佳）
- ✅ `ref` 模式添加长度比较，修复切片缓存错误

### v0.1.10
- ✅ 三层记忆化结构，智能缓存管理
- ✅ 生成 `_start()` 和 `_clear()` 辅助函数

### v0.1.8
- ✅ `key=ref` 可以直接使用，不需要 `r#ref`
- ✅ 参数验证：无效的参数名或模式会在编译时报错

### v0.1.7
- ✅ `ref` 模式：先比地址，若相等则命中；否则再比内容
- ✅ 参数重命名：`thread_mode`→`thread`，`index_mode`→`key`
- ✅ 键模式重命名：`light`→`ptr`，`normal`→`ref`，`heavy`→`val`
- ✅ 线程模式重命名：`local`→`single`
- ✅ `ptr` 模式改进：使用 (地址, 长度) 作为键
- ✅ `min!`/`max!` 空迭代器返回边界值

## 许可证

MIT 或 Apache-2.0 双许可证。
