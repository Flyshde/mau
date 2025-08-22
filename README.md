# Mau

一个用于 Rust 的过程宏库，提供内存化（memoization）功能和 MauQueue 优化。

## 功能特性

- **自动缓存**: 使用 `#[memo]` 宏自动为函数添加缓存功能
- **MauQueue 优化**: 通过 MauQueue 将复杂的循环逻辑转换为高效的代码
- **线程安全**: 内置线程安全的缓存机制
- **零运行时开销**: 编译时代码生成，无运行时性能损失

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
mau = "0.1.0"
```

## 使用示例

### 基础内存化

```rust
use mau::memo;

#[memo]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("{}", fibonacci(40)); // 高效计算，结果会被缓存
}
```

### MauQueue 优化

```rust
use mau::memo;

#[memo]
fn optimized_calculation(nums: &Vec<i32>, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

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

## 许可证

本项目采用 MIT 或 Apache-2.0 双许可证。
