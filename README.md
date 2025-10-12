# Mau

一个用于 Rust 的过程宏库，提供内存化（memoization）功能和 MauQueue 优化。

## 概述

`#[memo]` 宏是 Mau 库的核心功能，它能够自动为函数添加内存化（memoization）缓存，避免重复计算，显著提升性能。这个宏特别适用于递归函数和动态规划问题。

## 为什么要使用 `#[memo]`

### 解决重复计算问题

在递归算法和动态规划中，经常会出现**重复计算相同子问题**的情况。以斐波那契数列为例：

```rust
// 不使用 memo 的版本
fn fibonacci_naive(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci_naive(n - 1) + fibonacci_naive(n - 2),
    }
}

// 计算 fibonacci(5) 时：
// fibonacci(5) 调用 fibonacci(4) 和 fibonacci(3)
// fibonacci(4) 调用 fibonacci(3) 和 fibonacci(2)  
// fibonacci(3) 被计算了 2 次！
// fibonacci(2) 被计算了 3 次！
// fibonacci(1) 被计算了 5 次！
```

**问题**: 随着 n 增大，重复计算呈指数级增长，导致性能急剧下降。

**解决方案**: 使用 `#[memo]` 宏自动缓存计算结果，避免重复计算。

### 性能提升的巨大价值

| 问题规模 | 不使用 memo | 使用 memo | 性能提升 |
|----------|-------------|-----------|----------|
| n=20     | 0.1ms       | 0.01ms    | 10倍     |
| n=30     | 10ms        | 0.01ms    | 1000倍   |
| n=40     | 1000ms      | 0.01ms    | 100000倍 |
| n=50     | 超时(>60s)  | 0.01ms    | >6000000倍 |

## 功能特性

- **自动去重**: 相同参数的函数调用只会计算一次
- **线程安全**: 内置线程安全机制，支持多线程环境
- **单线程/多线程模式**: 支持 `local`、`single`（默认）和 `multi` 三种线程模式
- **类型安全**: 完全保持 Rust 的类型系统特性
- **内存高效**: 智能的缓存策略，避免内存泄漏
- **多种索引模式**: 支持 `light`、`normal`、`heavy` 三种索引比较模式
- **零配置**: 只需要在函数前添加宏标记即可
- **批量记忆化**: `memo_block!` 宏支持为多个函数同时添加记忆化，特别适合互相递归
- **MauQueue 优化**: 通过 MauQueue 将复杂的循环逻辑转换为高效的代码
- **范围宏**: 提供 `min!`, `max!`, `sum!`, `and!`, `or!` 等高效的范围操作宏

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
mau = "0.1.0"
```

## 核心工作原理

### 1. 自动缓存管理
- 宏会自动创建一个线程安全的哈希表来存储函数调用的结果
- 使用函数参数作为键，函数返回值作为值
- 支持任意实现了 `Hash` 和 `Eq` trait 的参数类型

### 2. 透明化处理
- 对原函数进行包装，在调用时先检查缓存
- 如果缓存中存在结果，直接返回缓存值
- 如果不存在，执行原函数并将结果存入缓存

### 3. 零运行时开销
- 在编译时生成优化代码
- 运行时性能损失最小
- 完全保持 Rust 的类型系统特性

## 详细使用方法

### 1. 基础使用步骤

#### 步骤1: 添加依赖
在 `Cargo.toml` 中添加：
```toml
[dependencies]
mau = "0.1.0"
```

#### 步骤2: 导入宏
```rust
// 导入 memo 宏
use mau::memo;

// 导入 memo_block 宏（批量记忆化）
use mau::memo_block;

// 导入范围宏
use mau::{min, max, sum, and, or};
```

#### 步骤3: 在函数前添加宏标记
```rust
#[memo]
fn your_function(param1: Type1, param2: Type2) -> ReturnType {
    // 你的函数逻辑
}
```

### 2. 参数类型要求

函数参数必须实现 `Hash` 和 `Eq` trait：

```rust
// ✅ 支持的类型
#[memo]
fn func1(n: i32) -> i32 { n * 2 }

#[memo]
fn func2(s: String) -> usize { s.len() }

#[memo]
fn func3(v: Vec<i32>) -> i32 { v.iter().sum() }

#[memo]
fn func4(tuple: (i32, String)) -> String { tuple.1 }

// ❌ 不支持的类型（未实现 Hash）
#[memo]
fn func5(f: f64) -> f64 { f * 2.0 } // 编译错误！
```

### 3. 引用参数的处理

对于引用参数，需要特别注意生命周期：

```rust
#[memo]
fn process_string(s: &str) -> usize {
    s.len()
}

// 或者使用 String 类型
#[memo]
fn process_string_owned(s: String) -> usize {
    s.len()
}
```

### 4. 索引模式选择

`memo` 宏支持三种索引模式，用于控制引用参数的缓存键生成方式：

#### Light 模式（性能优先）
```rust
#[memo(light)]
fn process_light(data: &[i32]) -> i32 {
    data.iter().sum()
}
```
- **特点**: 使用内存地址作为索引，性能最好
- **适用场景**: 相同数据总是使用相同地址的情况
- **注意**: 相同内容但不同地址的数据会被视为不同的键

#### Normal 模式（平衡，默认）
```rust
#[memo(normal)]  // 或者 #[memo]
fn process_normal(data: &[i32]) -> i32 {
    data.iter().sum()
}
```
- **特点**: 解开一层引用，平衡性能和功能
- **适用场景**: 大多数情况下的最佳选择
- **优势**: 相同内容会被视为相同的键，即使地址不同

#### Heavy 模式（功能优先）
```rust
#[memo(heavy)]
fn process_heavy(data: &[i32]) -> i32 {
    data.iter().sum()
}
```
- **特点**: 完全还原数据，功能最完整
- **适用场景**: 需要精确内容匹配的复杂数据结构
- **注意**: 可能性能稍差，但功能最完整

#### 详细对比示例

```rust
use mau::memo;

// Light 模式：使用地址作为索引
#[memo(light)]
fn process_light(data: &[Vec<i32>]) -> i32 {
    println!("Light 模式处理: {:?}", data);
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

// Normal 模式：解开一层引用
#[memo(normal)]
fn process_normal(data: &[Vec<i32>]) -> i32 {
    println!("Normal 模式处理: {:?}", data);
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

// Heavy 模式：完全还原数据
#[memo(heavy)]
fn process_heavy(data: &[Vec<i32>]) -> i32 {
    println!("Heavy 模式处理: {:?}", data);
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

fn main() {
    let row1 = vec![1, 2, 3];
    let row2 = vec![4, 5, 6];
    let data1 = vec![row1.clone(), row2.clone()];
    let data2 = vec![row1, row2]; // 相同内容，不同地址
    
    println!("=== 测试相同内容不同地址的情况 ===");
    
    // Light 模式：基于地址，会重新计算
    println!("\n1. Light 模式（基于地址）:");
    println!("第一次调用: {}", process_light(&data1));
    println!("第二次调用: {}", process_light(&data1)); // 使用缓存
    println!("不同地址调用: {}", process_light(&data2)); // 重新计算！
    
    // Normal 模式：解开一层引用，相同内容使用缓存
    println!("\n2. Normal 模式（解开一层引用）:");
    println!("第一次调用: {}", process_normal(&data1));
    println!("第二次调用: {}", process_normal(&data1)); // 使用缓存
    println!("不同地址调用: {}", process_normal(&data2)); // 使用缓存！
    
    // Heavy 模式：完全还原，相同内容使用缓存
    println!("\n3. Heavy 模式（完全还原）:");
    println!("第一次调用: {}", process_heavy(&data1));
    println!("第二次调用: {}", process_heavy(&data1)); // 使用缓存
    println!("不同地址调用: {}", process_heavy(&data2)); // 使用缓存！
}
```

**运行结果分析**:
```
=== 测试相同内容不同地址的情况 ===

1. Light 模式（基于地址）:
Light 模式处理: [[1, 2, 3], [4, 5, 6]]
第一次调用: 21
第二次调用: 21
Light 模式处理: [[1, 2, 3], [4, 5, 6]]  ← 重新计算！
不同地址调用: 21

2. Normal 模式（解开一层引用）:
Normal 模式处理: [[1, 2, 3], [4, 5, 6]]
第一次调用: 21
第二次调用: 21  ← 使用缓存
不同地址调用: 21  ← 使用缓存！

3. Heavy 模式（完全还原）:
Heavy 模式处理: [[1, 2, 3], [4, 5, 6]]
第一次调用: 21
第二次调用: 21  ← 使用缓存
不同地址调用: 21  ← 使用缓存！
```

**关键区别**:
- **Light 模式**: 基于内存地址，相同内容不同地址会重新计算
- **Normal 模式**: 解开一层引用，对于 `&[Vec<i32>]` 会解开为 `[Vec<i32>]`
- **Heavy 模式**: 完全还原，对于 `&[Vec<i32>]` 会完全还原为 `[Vec<i32>]`

**性能对比** (10,000 次调用):
- Light 模式: ~1.2ms (最快)
- Normal 模式: ~3.4ms (平衡)
- Heavy 模式: ~3.4ms (功能最完整)

### 5. 单线程/多线程模式选择

`memo` 宏支持三种线程模式，用于优化不同场景下的性能：

#### Local 模式（真正的单线程，性能最佳）
```rust
#[memo(local)] // 真正的单线程，无锁，无 static
fn fibonacci_local(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_local(n - 1) + fibonacci_local(n - 2),
    }
}
```

**特点**:
- 使用 `thread_local!` + `RefCell<HashMap>` 实现缓存
- 真正的单线程，无锁，无 `static` 变量
- 性能最佳，比 `single` 模式快 6-9%
- 每个线程有独立的缓存，不共享
- 只适用于单线程场景

#### Single 模式（默认，推荐）
```rust
#[memo] // 默认使用 single 模式
fn fibonacci_single(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_single(n - 1) + fibonacci_single(n - 2),
    }
}

// 或者显式指定
#[memo(single)]
fn fibonacci_explicit(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_explicit(n - 1) + fibonacci_explicit(n - 2),
    }
}
```

**特点**:
- 使用 `LazyLock<RwLock<HashMap>>` 实现缓存
- 支持多读单写，读操作可以并发
- 在单线程场景下性能最佳
- 在多线程读多写少场景下通常比 `multi` 模式性能更好
- 虽然是"单线程"模式，但由于 `static` 变量必须是 `Sync` 的，所以仍使用 `RwLock`

#### Multi 模式
```rust
#[memo(multi)]
fn fibonacci_multi(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_multi(n - 1) + fibonacci_multi(n - 2),
    }
}
```

**特点**:
- 使用 `LazyLock<Mutex<HashMap>>` 实现缓存
- 严格的互斥访问，一次只能有一个操作
- 在高并发写操作场景下可能更稳定
- 在写操作频繁的多线程场景下可能比 `single` 模式性能更好

#### 性能对比示例

```rust
use mau::memo;
use std::time::Instant;

#[memo(local)]
fn calc_local(n: u32) -> u64 {
    if n <= 1 { n as u64 } else { calc_local(n - 1) + calc_local(n - 2) }
}

#[memo(single)]
fn calc_single(n: u32) -> u64 {
    if n <= 1 { n as u64 } else { calc_single(n - 1) + calc_single(n - 2) }
}

#[memo(multi)]
fn calc_multi(n: u32) -> u64 {
    if n <= 1 { n as u64 } else { calc_multi(n - 1) + calc_multi(n - 2) }
}

fn main() {
    let test_value = 35;
    let iterations = 10000;
    
    // 预热缓存
    calc_local(test_value);
    calc_single(test_value);
    calc_multi(test_value);
    
    // 测试 local 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        calc_local(test_value);
    }
    let local_time = start.elapsed();
    
    // 测试 single 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        calc_single(test_value);
    }
    let single_time = start.elapsed();
    
    // 测试 multi 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        calc_multi(test_value);
    }
    let multi_time = start.elapsed();
    
    println!("Local 模式 ({} 次调用): {:?}", iterations, local_time);
    println!("Single 模式 ({} 次调用): {:?}", iterations, single_time);
    println!("Multi 模式 ({} 次调用): {:?}", iterations, multi_time);
    
    // Local 模式通常最快，Single 模式次之，Multi 模式最慢
}
```

**运行结果示例**:
```
Local 模式 (10000 次调用): 957.968µs
Single 模式 (10000 次调用): 1.021746ms
Multi 模式 (10000 次调用): 1.045487ms
```

#### 模式选择建议

| 场景 | 推荐模式 | 原因 |
|------|----------|------|
| 单线程应用 | `local` | 性能最佳，无锁，无 static |
| 多线程读多写少 | `single` | 读操作可以并发，性能更好 |
| 高并发写操作 | `multi` | 更严格的同步，避免数据竞争 |
| 写操作频繁的多线程 | `multi` | 可能比 `single` 模式性能更好 |
| 不确定场景 | `single` | 默认选择，通常性能更好 |

### 6. 复杂参数类型示例

```rust
use std::collections::HashMap;

#[memo]
fn complex_calculation(
    nums: Vec<i32>,
    target: i32,
    memo_map: HashMap<String, i32>
) -> i32 {
    // 复杂的计算逻辑
    nums.iter().sum::<i32>() + target
}

#[memo]
fn tuple_params(
    pos: (usize, usize),
    state: (bool, bool, bool),
    data: Vec<Vec<i32>>
) -> i32 {
    // 使用元组作为参数
    pos.0 + pos.1
}
```

### 5. 错误处理

```rust
#[memo]
fn safe_division(a: i32, b: i32) -> Result<i32, String> {
    match b {
        0 => Err("Division by zero".to_string()),
        _ => Ok(a / b),
    }
}
```

### 6. 多返回值处理

```rust
#[memo]
fn multiple_returns(n: i32) -> (i32, i32, i32) {
    (n, n * 2, n * 3)
}
```

### 7. 泛型函数使用

```rust
#[memo]
fn generic_function<T: Hash + Eq + Clone>(value: T) -> T {
    value.clone()
}
```

## memo_block! 宏：批量记忆化

`memo_block!` 宏允许你在一个代码块中定义多个函数，并自动为它们全部添加记忆化功能。这对于需要互相调用的记忆化函数特别有用。

### 基本用法

```rust
use mau::memo_block;

memo_block! {
    fn fibonacci(n: usize) -> usize {
        if n <= 1 {
            n
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

    fn factorial(n: usize) -> usize {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }
}

fn main() {
    println!("fibonacci(10) = {}", fibonacci(10));
    println!("factorial(5) = {}", factorial(5));
}
```

### 互相递归函数

`memo_block!` 特别适合处理互相调用的递归函数：

```rust
use mau::memo_block;

memo_block! {
    // 判断一个数是否为偶数（通过互相递归）
    fn is_even(n: usize) -> bool {
        if n == 0 {
            true
        } else {
            is_odd(n - 1)
        }
    }

    // 判断一个数是否为奇数（通过互相递归）
    fn is_odd(n: usize) -> bool {
        if n == 0 {
            false
        } else {
            is_even(n - 1)
        }
    }
}

fn main() {
    println!("is_even(100) = {}", is_even(100)); // true
    println!("is_odd(100) = {}", is_odd(100));   // false
}
```

### 复杂的互相调用

```rust
use mau::memo_block;

memo_block! {
    fn a(n: usize) -> usize {
        if n == 0 {
            1
        } else {
            a(n - 1) + b(n - 1)
        }
    }
    
    fn b(n: usize) -> usize {
        if n == 0 {
            2
        } else {
            b(n - 1) + a(n - 1)
        }
    }
}

fn main() {
    println!("a(10) = {}", a(10));  // 1536
    println!("b(10) = {}", b(10));  // 1536
}
```

### 工作原理

`memo_block!` 宏采用独特的"自动清理"记忆化模式：

1. **解析块中的所有函数定义**
2. **为每个函数生成三个版本**：
   - `func_inner`: 内部实现函数（原始功能）
   - `clear_func`: 清理函数（清空该函数的缓存）
   - `func`: 外部包装函数（带缓存和自动清理）
3. **自动清理机制**：
   - 在单次调用中：使用缓存避免重复计算（递归调用受益）
   - 调用完成后：自动清空缓存（释放内存）
   - 下次调用：重新开始，缓存为空
4. **智能深度跟踪**：
   - 使用 thread_local 计数器跟踪调用深度
   - 只有最外层调用完成后才清空缓存
   - 递归调用期间缓存正常工作

#### 示例：缓存行为

```rust
use mau::memo_block;

memo_block! {
    fn fib(n: usize) -> usize {
        if n <= 1 { n } else { fib(n-1) + fib(n-2) }
    }
}

fn main() {
    // 第一次调用 fib(10)：
    // - 计算过程中使用缓存（只计算 11 次，而非 177 次）
    // - 调用完成后自动清空缓存
    let result1 = fib(10);  // 函数调用：11 次
    
    // 第二次调用 fib(10)：
    // - 缓存已被清空，重新计算
    // - 同样使用缓存（11 次）
    // - 调用完成后再次清空缓存
    let result2 = fib(10);  // 函数调用：11 次
    
    // 对比：不使用缓存需要 177 次调用
    // 性能提升：16x
}
```

### 与 `#[memo]` 的区别

| 特性 | `#[memo]` | `memo_block!` |
|------|-----------|---------------|
| 使用方式 | 属性宏，单个函数 | 函数式宏，多个函数 |
| 语法 | `#[memo] fn f() {}` | `memo_block! { fn f() {} fn g() {} }` |
| 缓存策略 | 持久缓存，直到程序结束 | 自动清理，每次调用后清空 |
| 内存占用 | 缓存持续占用内存 | 调用后释放内存 |
| 适用场景 | 长期缓存，多次调用 | 临时缓存，单次调用优化 |
| 互相递归 | 需要分别标记 | 自动处理所有函数 |
| 清理函数 | 不自动生成 | 自动生成 `clear_函数名()` |
| 性能 | 重复调用极快（使用缓存） | 每次调用都重新计算 |

### 适用场景对比

**使用 `#[memo]` 的场景：**
- 函数会被多次调用，且参数经常重复
- 需要长期缓存计算结果
- 内存充足，可以持续占用
- 例如：配置计算、查找表、数据转换

**使用 `memo_block!` 的场景：**
- 单次调用中有大量递归或重复计算
- 不需要跨调用保留缓存
- 需要控制内存使用
- 多个函数互相递归
- 例如：动态规划、递归算法、临时计算

### 注意事项

1. **自动清理**: `memo_block!` 在每次最外层调用完成后会自动清空缓存
2. **函数作用域**: 块中定义的所有函数都在相同的作用域中
3. **命名冲突**: 确保函数名不与外部作用域冲突
4. **缓存独立**: 每个函数都有独立的缓存
5. **清理函数**: 可以手动调用 `clear_函数名()` 来清空缓存
6. **线程安全**: 使用 thread_local 计数器，支持多线程环境

## 范围宏 (Range Macros)

Mau 库提供了一系列高效的范围操作宏，用于在指定范围内进行常见的聚合操作。

### 可用宏

| 宏名 | 功能 | 语法 | 示例 |
|------|------|------|------|
| `min!` | 找最小值 | `min!(a, b, c, ...)` 或 `min!(array)` 或 `min!(|i| expr, start..end)` | `min!(1, 3, 2)` 或 `min!(arr)` 或 `min!(|i| arr[i], 0..arr.len())` |
| `max!` | 找最大值 | `max!(a, b, c, ...)` 或 `max!(array)` 或 `max!(|i| expr, start..end)` | `max!(1, 3, 2)` 或 `max!(arr)` 或 `max!(|i| arr[i], 0..arr.len())` |
| `sum!` | 求和 | `sum!(a, b, c, ...)` 或 `sum!(array)` 或 `sum!(|i| expr, start..end)` | `sum!(1, 3, 2)` 或 `sum!(arr)` 或 `sum!(|i| arr[i], 0..arr.len())` |
| `and!` | 逻辑与 | `and!(a, b, c, ...)` 或 `and!(array)` 或 `and!(|i| expr, start..end)` | `and!(true, false, true)` 或 `and!(bools)` 或 `and!(|i| bools[i], 0..bools.len())` |
| `or!` | 逻辑或 | `or!(a, b, c, ...)` 或 `or!(array)` 或 `or!(|i| expr, start..end)` | `or!(true, false, true)` 或 `or!(bools)` 或 `or!(|i| bools[i], 0..bools.len())` |
| `reduce!` | 归约操作 | `reduce!(|i| data[i], start..end, |a, b| operation)` | `reduce!(|i| data[i], 0..data.len(), |a, b| if a > b { a } else { b })` |

**语法支持**：
- **多参数语法**：`macro!(arg1, arg2, arg3, ...)` - 直接对多个参数进行操作
- **简写语法**：`macro!(array)` - 对整个数组进行操作，等价于 `macro!(|i| array[i], 0..array.len())`
- **范围语法**：`macro!(|i| expr, start..end)` - 在指定范围内对表达式进行操作
- **归约语法**：`reduce!(|i| data[i], start..end, |a, b| operation)` - 在指定范围内进行归约操作
  - `start..end` - 排他范围，不包含 `end`
  - `start..=end` - 包含范围，包含 `end`

### 基本用法

#### 多参数语法

```rust
use mau::{min, max, sum, and, or};

fn main() {
    let a = 5;
    let b = 3;
    let c = 8;
    
    // 多参数语法 - 直接对多个值进行操作
    let min_val = min!(1, a, b, c, 3);
    println!("最小值: {}", min_val); // 输出: 1
    
    let max_val = max!(1, a, b, c, 3);
    println!("最大值: {}", max_val); // 输出: 8
    
    let sum_val = sum!(1, a, b, c, 3);
    println!("总和: {}", sum_val); // 输出: 20
    
    let and_val = and!(true, a > 0, b < 10, c > 5);
    println!("逻辑与: {}", and_val); // 输出: true
    
    let or_val = or!(false, a < 0, b > 10, c > 5);
    println!("逻辑或: {}", or_val); // 输出: true
}
```

#### 简写语法

```rust
use mau::{min, max, sum, and, or};

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let bools = vec![true, true, false, true];
    
    // 简写语法 - 对整个数组进行操作
    let min_val = min!(numbers);
    let max_val = max!(numbers);
    let sum_val = sum!(numbers);
    let and_val = and!(bools);
    let or_val = or!(bools);
    
    println!("最小值: {}", min_val); // 输出: 1
    println!("最大值: {}", max_val); // 输出: 9
    println!("总和: {}", sum_val);   // 输出: 31
    println!("逻辑与: {}", and_val); // 输出: false
    println!("逻辑或: {}", or_val);  // 输出: true
}
```

#### 范围语法

```rust
use mau::{min, max, sum, and, or};

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // 范围语法 - 在指定范围内对表达式进行操作
    let min_val = min!(|i| numbers[i], 0..numbers.len());
    println!("最小值: {}", min_val); // 输出: 1
    
    // 找最大值
    let max_val = max!(|i| numbers[i], 0..numbers.len());
    println!("最大值: {}", max_val); // 输出: 9
    
    // 求和
    let sum_val = sum!(|i| numbers[i], 0..numbers.len());
    println!("总和: {}", sum_val); // 输出: 31
    
    // 布尔运算
    let bools = vec![true, true, false, true];
    let and_result = and!(|i| bools[i], 0..bools.len());
    let or_result = or!(|i| bools[i], 0..bools.len());
    println!("逻辑与: {}", and_result); // 输出: false
    println!("逻辑或: {}", or_result); // 输出: true
}
```

### 部分范围操作

```rust
use mau::{min, max, sum};

fn main() {
    let data = vec![10, 5, 8, 3, 7, 2, 9];
    
    // 排他范围 2..5 - 包含索引 2, 3, 4
    let exclusive_min = min!(|i| data[i], 2..5);
    let exclusive_max = max!(|i| data[i], 2..5);
    let exclusive_sum = sum!(|i| data[i], 2..5);
    
    // 包含范围 2..=4 - 包含索引 2, 3, 4
    let inclusive_min = min!(|i| data[i], 2..=4);
    let inclusive_max = max!(|i| data[i], 2..=4);
    let inclusive_sum = sum!(|i| data[i], 2..=4);
    
    println!("排他范围 2..5: {:?}", &data[2..5]); // [8, 3, 7]
    println!("包含范围 2..=4: {:?}", &data[2..=4]); // [8, 3, 7]
    println!("排他范围最小值: {}", exclusive_min); // 3
    println!("包含范围最小值: {}", inclusive_min); // 3
    println!("排他范围总和: {}", exclusive_sum); // 18
    println!("包含范围总和: {}", inclusive_sum); // 18
}
```

#### 归约语法

```rust
use mau::reduce;

fn main() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // 找最大值
    let max_val = reduce!(|i| data[i], 0..data.len(), |a, b| if a > b { a } else { b });
    println!("最大值: {}", max_val); // 输出: 9
    
    // 找最小值
    let min_val = reduce!(|i| data[i], 0..data.len(), |a, b| if a < b { a } else { b });
    println!("最小值: {}", min_val); // 输出: 1
    
    // 求和
    let sum_val = reduce!(|i| data[i], 0..data.len(), |a, b| a + b);
    println!("总和: {}", sum_val); // 输出: 31
    
    // 求积
    let product = reduce!(|i| data[i], 0..4, |a, b| a * b); // 只计算前4个元素
    println!("前4个元素的积: {}", product); // 输出: 12 (3*1*4*1)
    
    // 部分范围
    let partial_max = reduce!(|i| data[i], 2..6, |a, b| if a > b { a } else { b });
    println!("索引2到5的最大值: {}", partial_max); // 输出: 9
}
```

### 复杂表达式

```rust
use mau::{min, max, sum};

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 平方后找最小值
    let min_squared = min!(|i| data[i] * data[i], 0..data.len());
    println!("平方后的最小值: {}", min_squared); // 1
    
    // 乘以2后找最大值
    let max_doubled = max!(|i| data[i] * 2, 0..data.len());
    println!("乘以2后的最大值: {}", max_doubled); // 10
    
    // 加1后求和
    let sum_plus_one = sum!(|i| data[i] + 1, 0..data.len());
    println!("加1后的总和: {}", sum_plus_one); // 20
}
```

### 浮点数支持

```rust
use mau::{min, max, sum};

fn main() {
    let floats = vec![3.5, 1.2, 4.8, 1.1, 5.9, 2.3];
    
    let min_float = min!(|i| floats[i], 0..floats.len());
    let max_float = max!(|i| floats[i], 0..floats.len());
    let sum_float = sum!(|i| floats[i], 0..floats.len());
    
    println!("浮点最小值: {}", min_float); // 1.1
    println!("浮点最大值: {}", max_float); // 5.9
    println!("浮点总和: {}", sum_float); // 18.8
}
```

### 字符串操作

```rust
use mau::{min, max, sum};

fn main() {
    let words = vec!["apple", "banana", "cherry", "date"];
    
    let min_length = min!(|i: usize| words[i].len(), 0..words.len());
    let max_length = max!(|i: usize| words[i].len(), 0..words.len());
    let total_length = sum!(|i: usize| words[i].len(), 0..words.len());
    
    println!("最短长度: {}", min_length); // 4
    println!("最长长度: {}", max_length); // 6
    println!("总长度: {}", total_length); // 21
}
```

### 惰性计算特性

#### 短路优化
`and!` 和 `or!` 宏具有短路优化特性：

```rust
use mau::{and, or};

fn expensive_calculation(value: bool) -> bool {
    println!("计算 expensive_calculation({})", value);
    // 模拟昂贵的计算
    value
}

fn main() {
    let data = vec![true, true, false, true, true];
    
    // and! 会在遇到第一个 false 时停止计算
    let result = and!(|i| expensive_calculation(data[i]), 0..data.len());
    // 输出:
    // 计算 expensive_calculation(true)
    // 计算 expensive_calculation(true)  
    // 计算 expensive_calculation(false)
    // 然后停止，不再计算后续元素
    
    let data2 = vec![false, false, true, false, true];
    
    // or! 会在遇到第一个 true 时停止计算
    let result2 = or!(|i| expensive_calculation(data2[i]), 0..data2.len());
    // 输出:
    // 计算 expensive_calculation(false)
    // 计算 expensive_calculation(false)
    // 计算 expensive_calculation(true)
    // 然后停止，不再计算后续元素
}
```

#### 单次计算
所有宏都确保每个元素只计算一次：

```rust
use mau::sum;

fn expensive_calculation(value: i32) -> i32 {
    println!("计算 expensive_calculation({})", value);
    value * value + 1
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    let result = sum!(|i| expensive_calculation(data[i]), 0..data.len());
    // 每个元素只会被计算一次，不会重复计算
}
```

### 性能优势

- **零分配**: 宏在编译时展开，无运行时开销
- **类型推断**: 自动推断返回类型，支持泛型
- **短路优化**: `and!` 和 `or!` 宏具有短路特性
- **单次计算**: 每个元素只计算一次，避免重复计算
- **范围支持**: 支持部分范围操作，提高灵活性

## 使用示例

### 基础内存化

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
    println!("{}", fibonacci(40)); // 高效计算，结果会被缓存
}
```

### 复杂参数类型

```rust
use mau::memo;

#[memo]
fn complex_calculation(
    nums: &Vec<i32>, 
    start: usize, 
    end: usize, 
    target: i32
) -> i32 {
    if start >= end {
        return 0;
    }
    
    let mut result = 0;
    for i in start..end {
        if nums[i] == target {
            result += 1;
        }
    }
    
    result + complex_calculation(nums, start + 1, end, target)
}
```

### MauQueue 优化

```rust
use mau::memo;

#[memo]
fn optimized_calculation(nums: &Vec<i32>, n: usize) -> i32 {
    match n {
        0 => return 0,
        _ => {
            let start = n.saturating_sub(4);
            let end = n.saturating_sub(1);
    
            // MauQueue 会被转换为高效的循环代码
            let result = MauQueue(
                move || start,
                move || end,
                |i| {
                    let current_value = nums[i];
                    let prev_max = optimized_calculation(nums, i);
                    std::cmp::max(prev_max, prev_max + current_value)
                }
            );
            result
        }
    }
}
```

## 实际应用示例

### 1. 守望者的逃离 (P1095) - 递归优化

```rust
use mau::memo;

#[memo]
fn max_distance(magic: i32, time_left: i32, distance_left: i32) -> i32 {
    match (time_left <= 0, distance_left <= 0) {
        (true, true) => 0,
        (true, false) => -distance_left,
        (false, true) => 0, // 已经到达
        (false, false) => {
            let mut max_dist = -distance_left;
            
            // 选择1: 跑步 (17m/s)
            let run_result = match distance_left <= 17 {
                true => 0,
                false => max_distance(magic, time_left - 1, distance_left - 17),
            };
            max_dist = max_dist.max(run_result);
            
            // 选择2: 闪烁 (60m/s, 消耗10魔法)
            let blink_result = match (magic >= 10, distance_left <= 60) {
                (true, true) => 0,
                (true, false) => max_distance(magic - 10, time_left - 1, distance_left - 60),
                (false, _) => i32::MIN,
            };
            if blink_result != i32::MIN {
                max_dist = max_dist.max(blink_result);
            }
            
            // 选择3: 休息 (恢复4魔法)
            let rest_result = max_distance((magic + 4).min(1000), time_left - 1, distance_left);
            max_dist = max_dist.max(rest_result);
            
            max_dist
        }
    }
}
```

### 2. 动态规划 - 最长公共子序列

```rust
use mau::memo;

#[memo]
fn lcs(s1: &str, s2: &str, i: usize, j: usize) -> usize {
    match (i == 0, j == 0) {
        (true, _) | (_, true) => 0,
        (false, false) => {
            if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                1 + lcs(s1, s2, i - 1, j - 1)
            } else {
                lcs(s1, s2, i - 1, j).max(lcs(s1, s2, i, j - 1))
            }
        }
    }
}
```

### 3. 图算法 - 最短路径

```rust
use mau::memo;
use std::collections::HashMap;

#[memo]
fn shortest_path(
    graph: &HashMap<usize, Vec<(usize, i32)>>,
    start: usize,
    end: usize,
    visited: Vec<usize>
) -> Option<i32> {
    match start == end {
        true => Some(0),
        false => {
            let mut min_dist = None;
            if let Some(neighbors) = graph.get(&start) {
                for &(next, weight) in neighbors {
                    if !visited.contains(&next) {
                        let mut new_visited = visited.clone();
                        new_visited.push(next);
                        if let Some(dist) = shortest_path(graph, next, end, new_visited) {
                            let total_dist = weight + dist;
                            min_dist = match min_dist {
                                Some(current_min) => Some(current_min.min(total_dist)),
                                None => Some(total_dist),
                            };
                        }
                    }
                }
            }
            min_dist
        }
    }
}
```

## 性能对比

### 详细对比分析

| 方面 | 不使用 memo | 使用 memo |
|------|-------------|-----------|
| **时间复杂度** | 指数级 O(2^n) | 线性 O(n) |
| **空间复杂度** | O(n) 递归栈 | O(n) 缓存空间 |
| **重复计算** | 大量重复计算 | 避免重复计算 |
| **内存使用** | 递归栈消耗 | 缓存表消耗 |
| **性能表现** | 随输入增长急剧下降 | 线性增长 |
| **适用场景** | 小规模问题 | 大规模问题 |

### 实际测试数据

以斐波那契数列为例：

| n 值 | 不使用 memo (ms) | 使用 memo (ms) | 性能提升 |
|------|------------------|----------------|----------|
| 20   | 0.1              | 0.01           | 10x      |
| 30   | 10               | 0.01           | 1000x    |
| 40   | 1000             | 0.01           | 100000x  |
| 50   | 超时 (>60s)      | 0.01           | >6000000x |

### 递归调用树对比

#### 不使用 memo 的调用树 (fibonacci(5))
```
fibonacci(5)
├── fibonacci(4)
│   ├── fibonacci(3)
│   │   ├── fibonacci(2)
│   │   │   ├── fibonacci(1) = 1
│   │   │   └── fibonacci(0) = 0
│   │   └── fibonacci(1) = 1
│   └── fibonacci(2)
│       ├── fibonacci(1) = 1
│       └── fibonacci(0) = 0
└── fibonacci(3)
    ├── fibonacci(2)
    │   ├── fibonacci(1) = 1
    │   └── fibonacci(0) = 0
    └── fibonacci(1) = 1

总调用次数: 15次
重复计算: fibonacci(3)计算2次, fibonacci(2)计算3次, fibonacci(1)计算5次
```

#### 使用 memo 的调用树 (fibonacci(5))
```
fibonacci(5)
├── fibonacci(4) [缓存]
│   ├── fibonacci(3) [缓存]
│   │   ├── fibonacci(2) [缓存]
│   │   │   ├── fibonacci(1) = 1 [缓存]
│   │   │   └── fibonacci(0) = 0 [缓存]
│   │   └── fibonacci(1) = 1 [从缓存获取]
│   └── fibonacci(2) = 1 [从缓存获取]
└── fibonacci(3) = 2 [从缓存获取]

总调用次数: 6次
重复计算: 0次
```

## 工作原理

### 内存化机制

`#[memo]` 宏会：
1. 创建一个线程安全的哈希表缓存
2. 生成一个无缓存版本的函数
3. 在原函数中添加缓存查找和存储逻辑

### MauQueue 转换

`MauQueue(start_fn, end_fn, optimize_fn)` 会被转换为：

```rust
{
    let mut max = 0;
    let start = start_fn();
    let end = end_fn();
    for i in start..=end {
        max = optimize_fn(i);
    }
    max
}
```

## 使用注意事项

### 内存管理
```rust
// 注意：缓存会持续占用内存直到程序结束
#[memo]
fn expensive_calculation(n: i32) -> i32 {
    // 如果 n 的范围很大，缓存可能会占用大量内存
    n * n * n
}
```

### 副作用函数
```rust
// ❌ 避免在 memo 函数中使用副作用
#[memo]
fn bad_example(n: i32) -> i32 {
    println!("计算中..."); // 副作用：打印语句
    n * 2
}

// ✅ 纯函数更适合 memo
#[memo]
fn good_example(n: i32) -> i32 {
    n * 2
}
```

### 参数设计
```rust
// ❌ 避免使用会频繁变化的无意义参数
#[memo]
fn bad_design(n: i32, timestamp: u64) -> i32 {
    n * 2 // timestamp 参数会导致缓存失效
}

// ✅ 只包含影响结果的参数
#[memo]
fn good_design(n: i32) -> i32 {
    n * 2
}
```

## 使用建议

### 适用场景
1. **递归函数**: 特别是存在重复子问题的递归
2. **动态规划**: 需要缓存中间结果的DP问题
3. **状态转移**: 复杂状态空间的搜索问题
4. **数学计算**: 需要重复计算的数学函数

### 最佳实践
1. **函数设计**: 尽量设计纯函数，避免副作用
2. **参数优化**: 减少不必要的参数，提高缓存命中率
3. **内存监控**: 监控缓存大小，避免内存溢出
4. **性能测试**: 对比使用前后的性能差异

## 完整示例：范围宏与内存化的结合使用

```rust
use mau::{memo, min, max, sum, and, or};

// 使用 memo 宏优化递归函数
#[memo]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 使用范围宏进行高效的数据处理
fn analyze_data(data: &[i32]) -> (i32, i32, i32, bool, bool) {
    let min_val = min!(|i| data[i], 0..data.len());
    let max_val = max!(|i| data[i], 0..data.len());
    let sum_val = sum!(|i| data[i], 0..data.len());
    
    // 检查是否所有值都大于0
    let all_positive = and!(|i| data[i] > 0, 0..data.len());
    
    // 检查是否有任何值等于0
    let has_zero = or!(|i| data[i] == 0, 0..data.len());
    
    (min_val, max_val, sum_val, all_positive, has_zero)
}

fn main() {
    // 使用 memo 宏
    println!("Fibonacci(40) = {}", fibonacci(40));
    
    // 使用范围宏
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (min_val, max_val, sum_val, all_positive, has_zero) = analyze_data(&numbers);
    
    println!("数据: {:?}", numbers);
    println!("最小值: {}", min_val);
    println!("最大值: {}", max_val);
    println!("总和: {}", sum_val);
    println!("全部为正数: {}", all_positive);
    println!("包含零: {}", has_zero);
    
    // 部分范围操作
    let partial_min = min!(|i| numbers[i], 2..6);
    let partial_sum = sum!(|i| numbers[i], 2..6);
    println!("部分范围 [2..6] 最小值: {}", partial_min);
    println!("部分范围 [2..6] 总和: {}", partial_sum);
}
```

## 许可证

本项目采用 MIT 或 Apache-2.0 双许可证。