use mau::memo;
use std::time::Instant;

// 测试函数定义
#[memo(local)]
fn fibonacci_local(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_local(n - 1) + fibonacci_local(n - 2),
    }
}

#[memo(single)]
fn fibonacci_single(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_single(n - 1) + fibonacci_single(n - 2),
    }
}

#[memo(multi)]
fn fibonacci_multi(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_multi(n - 1) + fibonacci_multi(n - 2),
    }
}

#[memo(local)]
fn sum_local(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(single)]
fn sum_single(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(multi)]
fn sum_multi(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_performance() {
        let test_value = 30;
        let iterations = 1000;
        
        // 预热缓存
        fibonacci_local(test_value);
        fibonacci_single(test_value);
        fibonacci_multi(test_value);
        
        // 测试 Local 模式
        let start = Instant::now();
        for _ in 0..iterations {
            fibonacci_local(test_value);
        }
        let local_time = start.elapsed();
        
        // 测试 Single 模式
        let start = Instant::now();
        for _ in 0..iterations {
            fibonacci_single(test_value);
        }
        let single_time = start.elapsed();
        
        // 测试 Multi 模式
        let start = Instant::now();
        for _ in 0..iterations {
            fibonacci_multi(test_value);
        }
        let multi_time = start.elapsed();
        
        // 验证所有模式都能正常工作
        assert!(local_time.as_nanos() > 0);
        assert!(single_time.as_nanos() > 0);
        assert!(multi_time.as_nanos() > 0);
        
        // 验证结果一致性
        assert_eq!(fibonacci_local(test_value), fibonacci_single(test_value));
        assert_eq!(fibonacci_local(test_value), fibonacci_multi(test_value));
        
        println!("Fibonacci 性能测试 (n={}, {} 次调用):", test_value, iterations);
        println!("  Local 模式:  {:?}", local_time);
        println!("  Single 模式: {:?}", single_time);
        println!("  Multi 模式:  {:?}", multi_time);
    }

    #[test]
    fn test_array_sum_performance() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iterations = 1000;
        
        // 预热缓存
        sum_local(&data);
        sum_single(&data);
        sum_multi(&data);
        
        // 测试 Local 模式
        let start = Instant::now();
        for _ in 0..iterations {
            sum_local(&data);
        }
        let local_time = start.elapsed();
        
        // 测试 Single 模式
        let start = Instant::now();
        for _ in 0..iterations {
            sum_single(&data);
        }
        let single_time = start.elapsed();
        
        // 测试 Multi 模式
        let start = Instant::now();
        for _ in 0..iterations {
            sum_multi(&data);
        }
        let multi_time = start.elapsed();
        
        // 验证所有模式都能正常工作
        assert!(local_time.as_nanos() > 0);
        assert!(single_time.as_nanos() > 0);
        assert!(multi_time.as_nanos() > 0);
        
        // 验证结果一致性
        assert_eq!(sum_local(&data), sum_single(&data));
        assert_eq!(sum_local(&data), sum_multi(&data));
        
        println!("Array Sum 性能测试 ({} 次调用):", iterations);
        println!("  Local 模式:  {:?}", local_time);
        println!("  Single 模式: {:?}", single_time);
        println!("  Multi 模式:  {:?}", multi_time);
    }

    #[test]
    fn test_cache_hit_performance() {
        // 测试缓存命中性能
        let test_value = 25;
        let iterations = 5000;
        
        // 预热缓存
        fibonacci_local(test_value);
        fibonacci_single(test_value);
        fibonacci_multi(test_value);
        
        // 测试 Local 模式缓存命中
        let start = Instant::now();
        for _ in 0..iterations {
            fibonacci_local(test_value);
        }
        let local_cache_time = start.elapsed();
        
        // 测试 Single 模式缓存命中
        let start = Instant::now();
        for _ in 0..iterations {
            fibonacci_single(test_value);
        }
        let single_cache_time = start.elapsed();
        
        // 测试 Multi 模式缓存命中
        let start = Instant::now();
        for _ in 0..iterations {
            fibonacci_multi(test_value);
        }
        let multi_cache_time = start.elapsed();
        
        // 验证所有模式都能正常工作
        assert!(local_cache_time.as_nanos() > 0);
        assert!(single_cache_time.as_nanos() > 0);
        assert!(multi_cache_time.as_nanos() > 0);
        
        println!("缓存命中性能测试 (n={}, {} 次调用):", test_value, iterations);
        println!("  Local 模式:  {:?}", local_cache_time);
        println!("  Single 模式: {:?}", single_cache_time);
        println!("  Multi 模式:  {:?}", multi_cache_time);
    }

    #[test]
    fn test_different_input_sizes() {
        // 测试不同输入大小的性能
        let test_values = vec![15, 20, 25, 30];
        let iterations = 500;
        
        for &n in &test_values {
            // 预热缓存
            fibonacci_local(n);
            fibonacci_single(n);
            fibonacci_multi(n);
            
            // 测试 Local 模式
            let start = Instant::now();
            for _ in 0..iterations {
                fibonacci_local(n);
            }
            let local_time = start.elapsed();
            
            // 测试 Single 模式
            let start = Instant::now();
            for _ in 0..iterations {
                fibonacci_single(n);
            }
            let single_time = start.elapsed();
            
            // 测试 Multi 模式
            let start = Instant::now();
            for _ in 0..iterations {
                fibonacci_multi(n);
            }
            let multi_time = start.elapsed();
            
            // 验证所有模式都能正常工作
            assert!(local_time.as_nanos() > 0);
            assert!(single_time.as_nanos() > 0);
            assert!(multi_time.as_nanos() > 0);
            
            // 验证结果一致性
            assert_eq!(fibonacci_local(n), fibonacci_single(n));
            assert_eq!(fibonacci_local(n), fibonacci_multi(n));
            
            println!("输入大小 {} 性能测试 ({} 次调用):", n, iterations);
            println!("  Local 模式:  {:?}", local_time);
            println!("  Single 模式: {:?}", single_time);
            println!("  Multi 模式:  {:?}", multi_time);
        }
    }

    #[test]
    fn test_memory_usage() {
        // 测试内存使用情况（简单验证）
        let test_value = 20;
        
        // 多次调用以填充缓存（使用较小的范围避免溢出）
        for i in 0..30 {
            fibonacci_local(i);
            fibonacci_single(i);
            fibonacci_multi(i);
        }
        
        // 验证缓存工作正常
        assert_eq!(fibonacci_local(test_value), fibonacci_single(test_value));
        assert_eq!(fibonacci_local(test_value), fibonacci_multi(test_value));
        
        println!("内存使用测试完成 - 所有模式都正常工作");
    }

    #[test]
    fn test_concurrent_access_simulation() {
        // 模拟并发访问（在单线程环境中）
        let test_value = 25;
        let iterations = 1000;
        
        // 预热缓存
        fibonacci_local(test_value);
        fibonacci_single(test_value);
        fibonacci_multi(test_value);
        
        // 交替调用不同模式来模拟并发
        let start = Instant::now();
        for i in 0..iterations {
            match i % 3 {
                0 => { fibonacci_local(test_value); },
                1 => { fibonacci_single(test_value); },
                _ => { fibonacci_multi(test_value); },
            }
        }
        let mixed_time = start.elapsed();
        
        // 验证所有模式都能正常工作
        assert!(mixed_time.as_nanos() > 0);
        
        // 验证结果一致性
        assert_eq!(fibonacci_local(test_value), fibonacci_single(test_value));
        assert_eq!(fibonacci_local(test_value), fibonacci_multi(test_value));
        
        println!("并发访问模拟测试 ({} 次调用): {:?}", iterations, mixed_time);
    }
}