use mau::memo;

// Local 模式测试
#[memo(local)]
fn fibonacci_local(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_local(n - 1) + fibonacci_local(n - 2),
    }
}

#[memo(local)]
fn sum_local(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(local, light)]
fn sum_local_light(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(local, normal)]
fn sum_local_normal(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(local, heavy)]
fn sum_local_heavy(data: &[i32]) -> i32 {
    data.iter().sum()
}

// Single 模式测试
#[memo(single)]
fn fibonacci_single(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_single(n - 1) + fibonacci_single(n - 2),
    }
}

#[memo(single)]
fn sum_single(data: &[i32]) -> i32 {
    data.iter().sum()
}

// Multi 模式测试
#[memo(multi)]
fn fibonacci_multi(n: u32) -> u64 {
    match n {
        0 | 1 => n as u64,
        _ => fibonacci_multi(n - 1) + fibonacci_multi(n - 2),
    }
}

#[memo(multi)]
fn sum_multi(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_mode_basic() {
        // 测试 Local 模式的基本功能
        assert_eq!(fibonacci_local(0), 0);
        assert_eq!(fibonacci_local(1), 1);
        assert_eq!(fibonacci_local(10), 55);
        assert_eq!(fibonacci_local(20), 6765);
    }

    #[test]
    fn test_single_mode_basic() {
        // 测试 Single 模式的基本功能
        assert_eq!(fibonacci_single(0), 0);
        assert_eq!(fibonacci_single(1), 1);
        assert_eq!(fibonacci_single(10), 55);
        assert_eq!(fibonacci_single(20), 6765);
    }

    #[test]
    fn test_multi_mode_basic() {
        // 测试 Multi 模式的基本功能
        assert_eq!(fibonacci_multi(0), 0);
        assert_eq!(fibonacci_multi(1), 1);
        assert_eq!(fibonacci_multi(10), 55);
        assert_eq!(fibonacci_multi(20), 6765);
    }

    #[test]
    fn test_all_modes_consistency() {
        // 测试所有模式的结果一致性
        let test_values = vec![0, 1, 5, 10, 15, 20];
        
        for &n in &test_values {
            let local_result = fibonacci_local(n);
            let single_result = fibonacci_single(n);
            let multi_result = fibonacci_multi(n);
            
            assert_eq!(local_result, single_result);
            assert_eq!(local_result, multi_result);
        }
    }

    #[test]
    fn test_local_mode_caching() {
        // 测试 Local 模式的缓存功能
        let data1 = vec![1, 2, 3, 4, 5];
        let data2 = vec![1, 2, 3, 4, 5]; // 相同内容
        
        // 第一次调用
        let result1 = sum_local(&data1);
        assert_eq!(result1, 15);
        
        // 第二次调用相同数据，应该使用缓存
        let result2 = sum_local(&data1);
        assert_eq!(result2, 15);
        
        // 不同地址但相同内容的数据
        let result3 = sum_local(&data2);
        assert_eq!(result3, 15);
    }

    #[test]
    fn test_single_mode_caching() {
        // 测试 Single 模式的缓存功能
        let data1 = vec![1, 2, 3, 4, 5];
        let data2 = vec![1, 2, 3, 4, 5]; // 相同内容
        
        // 第一次调用
        let result1 = sum_single(&data1);
        assert_eq!(result1, 15);
        
        // 第二次调用相同数据，应该使用缓存
        let result2 = sum_single(&data1);
        assert_eq!(result2, 15);
        
        // 不同地址但相同内容的数据
        let result3 = sum_single(&data2);
        assert_eq!(result3, 15);
    }

    #[test]
    fn test_multi_mode_caching() {
        // 测试 Multi 模式的缓存功能
        let data1 = vec![1, 2, 3, 4, 5];
        let data2 = vec![1, 2, 3, 4, 5]; // 相同内容
        
        // 第一次调用
        let result1 = sum_multi(&data1);
        assert_eq!(result1, 15);
        
        // 第二次调用相同数据，应该使用缓存
        let result2 = sum_multi(&data1);
        assert_eq!(result2, 15);
        
        // 不同地址但相同内容的数据
        let result3 = sum_multi(&data2);
        assert_eq!(result3, 15);
    }

    #[test]
    fn test_local_mode_with_index_modes() {
        // 测试 Local 模式与不同索引模式的组合
        let data = vec![1, 2, 3, 4, 5];
        
        // Light 模式
        let result_light = sum_local_light(&data);
        assert_eq!(result_light, 15);
        
        // Normal 模式
        let result_normal = sum_local_normal(&data);
        assert_eq!(result_normal, 15);
        
        // Heavy 模式
        let result_heavy = sum_local_heavy(&data);
        assert_eq!(result_heavy, 15);
        
        // 所有模式结果应该一致
        assert_eq!(result_light, result_normal);
        assert_eq!(result_light, result_heavy);
    }

    #[test]
    fn test_local_mode_thread_isolation() {
        // 测试 Local 模式的线程隔离性
        // 注意：这个测试在单线程环境中运行，主要验证 Local 模式不会影响其他模式
        let data = vec![1, 2, 3, 4, 5];
        
        // Local 模式调用
        let local_result = sum_local(&data);
        
        // Single 模式调用
        let single_result = sum_single(&data);
        
        // Multi 模式调用
        let multi_result = sum_multi(&data);
        
        // 所有结果应该一致
        assert_eq!(local_result, single_result);
        assert_eq!(local_result, multi_result);
    }

    #[test]
    fn test_performance_comparison() {
        // 性能对比测试（简单版本）
        let test_value = 25;
        let iterations = 1000;
        
        // 预热缓存
        fibonacci_local(test_value);
        fibonacci_single(test_value);
        fibonacci_multi(test_value);
        
        // 测试 Local 模式
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            fibonacci_local(test_value);
        }
        let local_time = start.elapsed();
        
        // 测试 Single 模式
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            fibonacci_single(test_value);
        }
        let single_time = start.elapsed();
        
        // 测试 Multi 模式
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            fibonacci_multi(test_value);
        }
        let multi_time = start.elapsed();
        
        // 验证所有模式都能正常工作
        assert!(local_time.as_nanos() > 0);
        assert!(single_time.as_nanos() > 0);
        assert!(multi_time.as_nanos() > 0);
        
        // 在单线程环境中，Local 模式通常应该是最快的
        // 但这里只验证它们都能正常工作，不强制性能要求
        println!("Local 模式时间: {:?}", local_time);
        println!("Single 模式时间: {:?}", single_time);
        println!("Multi 模式时间: {:?}", multi_time);
    }

    #[test]
    fn test_complex_data_types() {
        // 测试复杂数据类型
        let data = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        
        #[memo(local)]
        fn sum_nested_local(data: &[Vec<i32>]) -> i32 {
            data.iter().map(|row| row.iter().sum::<i32>()).sum()
        }
        
        #[memo(single)]
        fn sum_nested_single(data: &[Vec<i32>]) -> i32 {
            data.iter().map(|row| row.iter().sum::<i32>()).sum()
        }
        
        #[memo(multi)]
        fn sum_nested_multi(data: &[Vec<i32>]) -> i32 {
            data.iter().map(|row| row.iter().sum::<i32>()).sum()
        }
        
        let local_result = sum_nested_local(&data);
        let single_result = sum_nested_single(&data);
        let multi_result = sum_nested_multi(&data);
        
        assert_eq!(local_result, 21); // 1+2+3+4+5+6
        assert_eq!(local_result, single_result);
        assert_eq!(local_result, multi_result);
    }

    #[test]
    fn test_error_handling() {
        // 测试错误处理
        #[memo(local)]
        fn might_panic_local(n: u32) -> u32 {
            if n == 0 {
                panic!("Zero not allowed");
            }
            n * 2
        }
        
        #[memo(single)]
        fn might_panic_single(n: u32) -> u32 {
            if n == 0 {
                panic!("Zero not allowed");
            }
            n * 2
        }
        
        // 测试正常情况
        assert_eq!(might_panic_local(5), 10);
        assert_eq!(might_panic_single(5), 10);
        
        // 测试异常情况
        let result = std::panic::catch_unwind(|| might_panic_local(0));
        assert!(result.is_err());
        
        let result = std::panic::catch_unwind(|| might_panic_single(0));
        assert!(result.is_err());
    }
}
