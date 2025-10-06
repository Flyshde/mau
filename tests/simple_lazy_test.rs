use mau::{and, or};

// 简单的计数器
static mut SIMPLE_CALL_COUNT: u32 = 0;

fn reset_simple_count() {
    unsafe { SIMPLE_CALL_COUNT = 0; }
}

fn get_simple_count() -> u32 {
    unsafe { SIMPLE_CALL_COUNT }
}

fn count_call() {
    unsafe { SIMPLE_CALL_COUNT += 1; }
}

fn expensive_bool(value: bool) -> bool {
    count_call();
    println!("  调用 expensive_bool({}) - 总调用次数: {}", value, get_simple_count());
    value
}

#[test]
fn test_and_short_circuit() {
    println!("=== 测试 and! 短路优化 ===");
    
    let data = vec![true, true, false, true, true];
    
    reset_simple_count();
    let result = and!(|i| expensive_bool(data[i]), 0..data.len());
    
    println!("结果: {}, 调用次数: {}", result, get_simple_count());
    
    // 应该在第3次调用后停止（遇到 false）
    assert_eq!(result, false);
    assert_eq!(get_simple_count(), 3);
}

#[test]
fn test_or_short_circuit() {
    println!("=== 测试 or! 短路优化 ===");
    
    let data = vec![false, false, true, false, true];
    
    reset_simple_count();
    let result = or!(|i| expensive_bool(data[i]), 0..data.len());
    
    println!("结果: {}, 调用次数: {}", result, get_simple_count());
    
    // 应该在第3次调用后停止（遇到 true）
    assert_eq!(result, true);
    assert_eq!(get_simple_count(), 3);
}

#[test]
fn test_and_all_true() {
    println!("=== 测试 and! 全为 true ===");
    
    let data = vec![true, true, true, true, true];
    
    reset_simple_count();
    let result = and!(|i| expensive_bool(data[i]), 0..data.len());
    
    println!("结果: {}, 调用次数: {}", result, get_simple_count());
    
    // 应该调用所有元素
    assert_eq!(result, true);
    assert_eq!(get_simple_count(), 5);
}

#[test]
fn test_or_all_false() {
    println!("=== 测试 or! 全为 false ===");
    
    let data = vec![false, false, false, false, false];
    
    reset_simple_count();
    let result = or!(|i| expensive_bool(data[i]), 0..data.len());
    
    println!("结果: {}, 调用次数: {}", result, get_simple_count());
    
    // 应该调用所有元素
    assert_eq!(result, false);
    assert_eq!(get_simple_count(), 5);
}
