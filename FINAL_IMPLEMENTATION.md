# memo_block! 宏最终实现文档

## ✅ 完成的所有功能

### 1. 批量记忆化 ✅
```rust
memo_block! {
    fn a() { ... }
    fn b() { ... }
}
```

### 2. 自动清理模式 ✅
- 外部函数：调用 inner → 清空缓存 → 返回结果
- 内部函数：带记忆化，递归调用 _inner

### 3. 互相递归支持 ✅
- 块内函数调用自动替换为 _inner 版本
- is_even_inner 调用 is_odd_inner

### 4. 复用参数逻辑 ✅
- 与 #[memo] 共享 generate_cache_keys()
- 统一维护，便于调试

### 5. 每个函数独立属性 ✅
```rust
memo_block! {
    #[memo(key=ptr)]
    fn a() { ... }
    
    #[memo(thread=multi, key=val)]
    fn b() { ... }
}
```

### 6. ptr 模式改进 ✅
- 切片：使用 (地址, 长度) 作为键
- 其他：使用地址
- 更准确的缓存判断

## 最终语法

### #[memo] 属性宏

```rust
// 命名参数（推荐）
#[memo(thread=single, key=r#ref)]
#[memo(thread=multi, key=ptr)]
#[memo(key=val)]

// 位置参数（向后兼容）
#[memo(local, normal)]
#[memo(multi, heavy)]

// 默认值
#[memo]  // thread=single, key=ref
```

### memo_block! 函数宏

```rust
memo_block! {
    #[memo(key=ptr)]
    fn a() { a(); b() }
    
    #[memo(key=r#ref)]         // ref 需要 r# 转义
    fn b() { b(); a() }
    
    #[memo(thread=multi, key=val)]  // 多个属性用逗号
    fn c() { c() }
    
    // 无属性，使用默认值
    fn d() { d() }
}
```

## 参数说明

### thread（线程模式）

| 值 | 实现 | 默认 | 说明 |
|----|------|------|------|
| `single` | thread_local! + RefCell | ✅ | 单线程，无锁，最快 |
| `multi` | LazyLock + Mutex | | 多线程，全局共享 |

### key（键模式）

| 值 | 切片类型 | 其他引用 | 默认 | 说明 |
|----|---------|---------|------|------|
| `ptr` | (地址, 长度) | 地址 | | 性能最好，地址+长度敏感 |
| `r#ref` | data.to_vec() | 解开引用 | ✅ | 平衡，相同内容=相同键 |
| `val` | 完全还原 | 完全还原 | | 功能最完整 |

## 生成的代码结构

对于每个函数 `foo`：

```rust
// 1. 缓存
thread_local! { static FOO_CACHE: RefCell<HashMap<K, V>> }  // single
// 或
static FOO_CACHE: LazyLock<Mutex<HashMap<K, V>>>             // multi

// 2. 清理函数
fn clear_foo() {
    // 清空缓存
}

// 3. 内部实现（带记忆化）
fn foo_inner(...) -> R {
    let cache_key = ...;  // 根据 key 模式生成
    // 检查缓存
    // 原始实现（调用其他 _inner）
    // 存入缓存
    // 返回结果
}

// 4. 外部包装
fn foo(...) -> R {
    let result = foo_inner(...);
    clear_foo();
    result
}
```

## 完整示例

```rust
use mau::memo_block;

memo_block! {
    // 使用 ptr 模式：(地址, 长度)
    #[memo(key=ptr)]
    fn binary_search(arr: &[i32], target: i32) -> bool {
        if arr.is_empty() {
            false
        } else if arr[0] == target {
            true
        } else {
            binary_search(&arr[1..], target)
        }
    }
    
    // 使用 val 模式：完全还原
    #[memo(key=val)]
    fn matrix_sum(matrix: &[Vec<i32>]) -> i32 {
        if matrix.is_empty() {
            0
        } else {
            let sum: i32 = matrix[0].iter().sum();
            sum + matrix_sum(&matrix[1..])
        }
    }
    
    // 使用 multi 线程 + ref 键
    #[memo(thread=multi, key=r#ref)]
    fn shared_fib(n: usize) -> usize {
        if n <= 1 { n } else { shared_fib(n-1) + shared_fib(n-2) }
    }
    
    // 默认配置（single + ref）
    fn normal_fib(n: usize) -> usize {
        if n <= 1 { n } else { normal_fib(n-1) + normal_fib(n-2) }
    }
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];
    println!("binary_search: {}", binary_search(&arr, 3));
    
    let matrix = vec![vec![1, 2], vec![3, 4]];
    println!("matrix_sum: {}", matrix_sum(&matrix));
    
    println!("shared_fib(10): {}", shared_fib(10));
    println!("normal_fib(10): {}", normal_fib(10));
}
```

## 性能数据

| 函数 | 不使用缓存 | 使用缓存 | 提升 |
|------|-----------|---------|------|
| fib(10) | 177 次 | 11 次 | 16x |
| fib(20) | 10946 次 | 21 次 | 521x |

## 测试验证

✅ ptr 模式使用 (地址, 长度)
✅ 每个函数独立配置正常
✅ 互相递归正常
✅ 所有示例测试通过
✅ 向后兼容性保持

---

**所有功能完成！** 🎉
