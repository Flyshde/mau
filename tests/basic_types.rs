use mau::memo;
use std::collections::HashMap;

#[test]
fn test_basic_types() {
    println!("=== Testing Basic Types ===");
    
    // 测试 i32 类型
    let result1 = basic_i32(42);
    let result2 = basic_i32(42); // 应该命中缓存
    assert_eq!(result1, result2);
    println!("i32 test: {} == {}", result1, result2);
    
    // 测试 usize 类型
    let result1 = basic_usize(100);
    let result2 = basic_usize(100); // 应该命中缓存
    assert_eq!(result1, result2);
    println!("usize test: {} == {}", result1, result2);
    
    // 测试 bool 类型
    let result1 = basic_bool(true);
    let result2 = basic_bool(true); // 应该命中缓存
    assert_eq!(result1, result2);
    println!("bool test: {} == {}", result1, result2);
    
    // 测试 char 类型
    let result1 = basic_char('A');
    let result2 = basic_char('A'); // 应该命中缓存
    assert_eq!(result1, result2);
    println!("char test: {} == {}", result1, result2);
    
    // 测试多个参数
    let result1 = basic_multi(10, 20, true);
    let result2 = basic_multi(10, 20, true); // 应该命中缓存
    assert_eq!(result1, result2);
    println!("multi params test: {} == {}", result1, result2);
}

#[memo]
fn basic_i32(n: i32) -> i32 {
    println!("Computing basic_i32({})", n);
    n * n + 1
}

#[memo]
fn basic_usize(n: usize) -> usize {
    println!("Computing basic_usize({})", n);
    n * n + 1
}

#[memo]
fn basic_bool(b: bool) -> i32 {
    println!("Computing basic_bool({})", b);
    if b { 42 } else { 0 }
}

#[memo]
fn basic_char(c: char) -> u32 {
    println!("Computing basic_char({})", c);
    c as u32
}

#[memo]
fn basic_multi(a: i32, b: i32, flag: bool) -> i32 {
    println!("Computing basic_multi({}, {}, {})", a, b, flag);
    if flag { a + b } else { a - b }
}
