# memo_block! 宏最终语法

## ✅ 简洁的属性语法

### 基本形式

只支持 `#[key=value]` 语法：

```rust
memo_block! {
    #[key=ptr]
    fn a(n: usize) -> usize { 
        if n == 0 { 1 } else { a(n-1) + b(n-1) }
    }
    
    #[key=r#ref]      // ref 是关键字，使用 r# 转义
    fn b(n: usize) -> usize { 
        if n == 0 { 2 } else { b(n-1) + a(n-1) }
    }
}
```

### 支持的参数

**thread（线程模式）：**
- `single`（默认）- thread_local，单线程，无锁
- `multi` - Mutex，多线程，全局共享

**key（键模式）：**
- `ptr` - 使用指针地址
- `r#ref`（默认）- 解开引用，**注意需要 r# 转义**
- `val` - 完全还原值

### 完整示例

```rust
use mau::memo_block;

memo_block! {
    // 使用指针键（性能优先）
    #[key=ptr]
    fn fast_lookup(data: &[i32], target: i32) -> bool {
        if data.is_empty() {
            false
        } else if data[0] == target {
            true
        } else {
            fast_lookup(&data[1..], target)
        }
    }
    
    // 使用引用键（默认，平衡）
    #[key=r#ref]
    fn balanced_search(arr: &[i32], x: i32) -> i32 {
        if arr.is_empty() { 0 } else { arr[0] + balanced_search(&arr[1..], x) }
    }
    
    // 使用值键（功能最完整）
    #[key=val]
    fn precise_calc(matrix: &[Vec<i32>]) -> i32 {
        if matrix.is_empty() {
            0
        } else {
            let sum: i32 = matrix[0].iter().sum();
            sum + precise_calc(&matrix[1..])
        }
    }
    
    // 多线程模式
    #[thread=multi]
    fn shared_compute(n: usize) -> usize {
        if n <= 1 { n } else { shared_compute(n-1) + shared_compute(n-2) }
    }
    
    // 组合使用
    #[thread=multi]
    #[key=ptr]
    fn multi_fast(data: &[i32]) -> i32 {
        data.iter().sum()
    }
    
    // 无属性（使用默认值：thread=single, key=ref）
    fn default_fn(n: usize) -> usize {
        if n <= 1 { n } else { default_fn(n-1) + default_fn(n-2) }
    }
}

fn main() {
    // 使用生成的函数
    println!("fast_lookup: {}", fast_lookup(&[1,2,3], 2));
    println!("shared_compute: {}", shared_compute(10));
    
    // 使用生成的清理函数
    clear_fast_lookup();
    clear_shared_compute();
    clear_default_fn();
}
```

## 语法规则

1. **属性格式**：`#[key=value]`
2. **关键字转义**：`ref` → `r#ref`
3. **多个属性**：可以有多个 `#[...]`
4. **默认值**：无属性时使用 `thread=single, key=ref`

## 生成的内容

对于每个函数 `foo`，生成：

```rust
// 1. 缓存（根据 thread 模式）
thread_local! { static FOO_CACHE: ... }  // single
// 或
static FOO_CACHE: ...                    // multi

// 2. 清理函数
fn clear_foo() { ... }

// 3. 内部实现（带记忆化）
fn foo_inner(...) {
    // 检查缓存（根据 key 模式生成键）
    // 原始实现（调用其他 _inner）
    // 存入缓存
}

// 4. 外部包装
fn foo(...) {
    let result = foo_inner(...);
    clear_foo();
    result
}
```

## 完整功能

✅ 批量记忆化
✅ 自动清理（调用后清空缓存）
✅ 互相递归（自动替换为 _inner）
✅ 每个函数独立配置
✅ 复用 #[memo] 的参数逻辑
✅ 简洁的 key=value 语法

---

**最终实现完成！** 🎉
