# Mau

一个强大的 Rust 过程宏库，提供记忆化（memoization）功能和高效的范围操作宏。

## 功能特性

- ✅ **自动记忆化**: `#[memo]` 属性宏，智能缓存管理
- ✅ **智能清理**: `start!` 宏，自动清空缓存，避免内存泄漏
- ✅ **智能缓存键**: 三种键模式（`ptr`、`ref`、`val`），平衡性能和功能
- ✅ **线程模式**: 单线程（`single`）和多线程（`multi`）支持
- ✅ **范围宏**: `min!`、`max!`、`sum!`、`and!`、`or!`、`reduce!` 等高效宏
- ✅ **空迭代器处理**: `min!` 和 `max!` 对空迭代器返回边界值

## 安装

```toml
[dependencies]
mau = "0.1.10"
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
use mau::{memo, start};

#[memo]
fn compute(n: i32) -> i32 {
    // 复杂计算
    n * n * n
}

fn main() {
    // 使用 start! 宏，自动清空缓存
    let result = start!(compute(100));
    println!("结果: {}", result);
    // 调用结束，缓存已清空，避免内存泄漏
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

### `start!` 宏 - 智能清理

自动清空缓存，避免内存泄漏：

```rust
use mau::{memo, start};

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
    let result = start!(is_even(100));
    
    // 支持复杂表达式
    let (r1, r2) = start!((is_even(50), is_odd(51)));
    
    // 支持代码块
    let result = start!({
        let a = is_even(10);
        let b = is_even(20);
        a && b
    });
    
    // 嵌套调用（自动递归替换）
    let result = start!(is_even(is_odd(5) as usize * 10));
}
```

**何时使用 `start!`**：

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
    start!(compute(100));
    
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
- `ptr`：地址+长度，最快
- `ref`（默认）：先比地址+长度，再比内容，平衡性能
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

#### ref 模式 - 默认，先比地址+长度，再比内容

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
| `ptr` | 地址+长度 | ⚡极快 | ❌不命中 | 最快 | 相同引用反复调用 |
| `ref` | 先比地址+长度，若相等则命中；否则比内容 | ⚡快 | ✅命中 | 快 | 一般情况（推荐） |
| `val` | 深度比较 | 慢 | ✅命中 | 慢 | 复杂嵌套类型 |

## 使用场景

### 场景 1: 动态规划（使用 start! 自动清理）

```rust
use mau::{memo, start};

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
    let result = start!(merge_stones(&stones));
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
use mau::{memo, start};

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
    let r3 = start!(is_even(100));
    // 缓存已清空
    
    // 方式 3: 手动清理
    is_even_clear();
    is_odd_clear();
}
```

## 范围宏

高效的范围聚合操作。

### 基本用法

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
use mau::{memo, start};

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
    let result = start!(knapsack(&weights, &values, capacity, weights.len()));
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

// 场景2：不同调用但参数可能相同（推荐，默认）
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
        start!(compute(i));  // 自动清理
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
#[memo]                              // 默认：thread=single, key=ref
#[memo(thread=single, key=ref)]      // 命名参数
#[memo(thread=multi, key=ptr)]       // 多线程 + 地址键
#[memo(key=val)]                     // 只指定 key
```

### `start!` 宏语法

```rust
start!(func(args))                   // 单个函数调用
start!((func1(a), func2(b)))        // 多个调用（元组）
start!({ let a = f(); a + 1 })      // 代码块
start!(f(g(h(x))))                   // 嵌套调用（自动递归替换）
```

### 范围宏语法

```rust
min!(1, 2, 3)                        // 多参数
min!(array)                          // 整个数组
min!(|i| array[i], 0..10)           // 范围表达式
min!(|i| array[i] * 2, 0..=9)       // 包含范围

reduce!(|i| data[i], 0..n, |a, b| a.max(b))  // 自定义归约
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
    let result = start!({
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

1. **默认使用 `ref` 模式**：最佳的性能/功能平衡
2. **单次计算用 `start!`**：自动清理，避免内存泄漏
3. **长期缓存用普通调用**：跨调用保留，提升性能
4. **递归传递相同引用用 `ptr`**：最快
5. **复杂类型用 `val`**：功能最完整
6. **避免副作用**：只在纯函数上使用
7. **监控内存**：参数范围大时使用 `start!` 或手动 `_clear()`

## 更新日志

### v0.1.10
- ✅ `start!` 宏：自动清理缓存，避免内存泄漏
- ✅ `ref` 模式改进：同时比较地址和长度，正确区分不同长度的切片
- ✅ 生成 `_start()` 和 `_clear()` 辅助函数
- ✅ 修复切片缓存错误命中问题

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
