use mau::memo;
use std::time::Instant;

#[test]
fn test_performance_comparison() {
    println!("=== Performance Comparison Test ===");
    
    // 测试不同规模的问题
    let test_cases = vec![
        (10, "Small"),
        (15, "Medium"),
        (20, "Large"),
    ];
    
    for (n, size) in test_cases {
        println!("\n--- {} Problem (n={}) ---", size, n);
        
        // 测试斐波那契数列
        test_fibonacci(n);
        
        // 测试阶乘 - 只测试小数值避免溢出
        if n <= 15 {
            test_factorial(n);
        }
        
        // 测试组合数
        test_combination(n, n / 2);
    }
}

fn test_fibonacci(n: u32) {
    println!("Testing Fibonacci({})", n);
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = fibonacci_no_memo(n);
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = fibonacci_memo(n);
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Fibonacci: {} == {} | Speedup: {:.2}x", result_no_memo, result_memo, speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Fibonacci: {} == {} | Overhead: {:.2}x", result_no_memo, result_memo, overhead);
    }
}

fn test_factorial(n: u32) {
    println!("Testing Factorial({})", n);
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = factorial_no_memo(n);
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = factorial_memo(n);
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Factorial: {} == {} | Speedup: {:.2}x", result_no_memo, result_memo, speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Factorial: {} == {} | Overhead: {:.2}x", result_no_memo, result_memo, overhead);
    }
}

fn test_combination(n: u32, k: u32) {
    println!("Testing Combination({}, {})", n, k);
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = combination_no_memo(n, k);
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = combination_memo(n, k);
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Combination: {} == {} | Speedup: {:.2}x", result_no_memo, result_memo, speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Combination: {} == {} | Overhead: {:.2}x", result_no_memo, result_memo, overhead);
    }
}

// 不使用 memo 的版本
fn fibonacci_no_memo(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_no_memo(n - 1) + fibonacci_no_memo(n - 2),
    }
}

fn factorial_no_memo(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial_no_memo(n - 1),
    }
}

fn combination_no_memo(n: u32, k: u32) -> u64 {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        combination_no_memo(n - 1, k - 1) + combination_no_memo(n - 1, k)
    }
}

// 使用 memo 的版本
#[memo]
fn fibonacci_memo(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memo(n - 1) + fibonacci_memo(n - 2),
    }
}

#[memo]
fn factorial_memo(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial_memo(n - 1),
    }
}

#[memo]
fn combination_memo(n: u32, k: u32) -> u64 {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        combination_memo(n - 1, k - 1) + combination_memo(n - 1, k)
    }
}
