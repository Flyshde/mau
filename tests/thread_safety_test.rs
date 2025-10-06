use mau::memo;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// 模拟一个计算密集的函数
#[memo]
fn expensive_calculation(n: i32) -> i32 {
    // 模拟一些计算时间
    thread::sleep(Duration::from_millis(10));
    n * n + 1
}

// 测试并发访问
#[memo]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[test]
fn test_thread_safety() {
    println!("=== 测试 memo 宏的线程安全性 ===");
    
    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                // 每个线程计算相同的值
                let result = expensive_calculation(5);
                println!("线程 {} 计算结果: {}", i, result);
                result
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    // 所有线程应该得到相同的结果
    let first_result = results[0];
    assert!(results.iter().all(|&r| r == first_result));
    assert_eq!(first_result, 26); // 5 * 5 + 1 = 26
    
    println!("所有线程结果一致: {:?}", results);
}

#[test]
fn test_concurrent_fibonacci() {
    println!("=== 测试并发斐波那契计算 ===");
    
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                let n = 10 + i; // 计算不同的值
                let result = fibonacci(n);
                println!("线程 {} 计算 fibonacci({}) = {}", i, n, result);
                (n, result)
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    // 验证结果正确性
    for (n, result) in &results {
        let expected = match n {
            10 => 55,
            11 => 89,
            12 => 144,
            13 => 233,
            14 => 377,
            _ => panic!("意外的 n 值: {}", n),
        };
        assert_eq!(*result, expected);
    }
    
    println!("并发斐波那契计算结果: {:?}", results);
}

#[test]
fn test_shared_cache_access() {
    println!("=== 测试共享缓存访问 ===");
    
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&shared_data);
            thread::spawn(move || {
                // 每个线程访问相同的缓存
                let result = expensive_calculation(3);
                println!("线程 {} 使用共享数据 {:?} 计算结果: {}", i, data, result);
                result
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    // 所有线程应该得到相同的结果
    assert!(results.iter().all(|&r| r == 10)); // 3 * 3 + 1 = 10
    println!("共享缓存访问结果: {:?}", results);
}

#[test]
fn test_race_condition_prevention() {
    println!("=== 测试竞态条件防护 ===");
    
    // 使用多个线程同时计算相同的值
    let handles: Vec<_> = (0..20)
        .map(|_| {
            thread::spawn(|| {
                // 所有线程计算相同的值
                expensive_calculation(7)
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect();
    
    // 验证没有竞态条件，所有结果都相同
    let first_result = results[0];
    assert!(results.iter().all(|&r| r == first_result));
    assert_eq!(first_result, 50); // 7 * 7 + 1 = 50
    
    println!("竞态条件防护测试通过，所有 {} 个线程结果一致", results.len());
}

