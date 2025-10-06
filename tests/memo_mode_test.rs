use mau::memo;

// 测试 single 模式（默认）
#[memo]
fn fibonacci_single(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_single(n - 1) + fibonacci_single(n - 2),
    }
}

// 测试 multi 模式
#[memo(multi)]
fn fibonacci_multi(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_multi(n - 1) + fibonacci_multi(n - 2),
    }
}

// 测试带参数的 single 模式
#[memo(single)]
fn factorial_single(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial_single(n - 1),
    }
}

// 测试带参数的 multi 模式
#[memo(multi)]
fn factorial_multi(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial_multi(n - 1),
    }
}

#[test]
fn test_single_mode() {
    // 测试 single 模式
    let result = fibonacci_single(10);
    assert_eq!(result, 55);
    
    let result2 = factorial_single(5);
    assert_eq!(result2, 120);
}

#[test]
fn test_multi_mode() {
    // 测试 multi 模式
    let result = fibonacci_multi(10);
    assert_eq!(result, 55);
    
    let result2 = factorial_multi(5);
    assert_eq!(result2, 120);
}

#[test]
fn test_default_mode() {
    // 测试默认模式（应该是 single）
    let result = fibonacci_single(15);
    assert_eq!(result, 610);
}

#[test]
fn test_performance_comparison() {
    use std::time::Instant;
    
    // 多次测试以获得更准确的结果
    let iterations = 10;
    
    // 测试 single 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _result_single = fibonacci_single(25);
    }
    let single_duration = start.elapsed();
    
    // 测试 multi 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        let _result_multi = fibonacci_multi(25);
    }
    let multi_duration = start.elapsed();
    
    println!("Single mode ({} iterations): {:?}", iterations, single_duration);
    println!("Multi mode ({} iterations): {:?}", iterations, multi_duration);
    
    // 验证两种模式都能正常工作
    assert_eq!(fibonacci_single(25), fibonacci_multi(25));
    
    // 注意：由于缓存的存在，第二次调用会很快，所以性能差异可能不明显
    // 这里我们只验证功能正确性
}
