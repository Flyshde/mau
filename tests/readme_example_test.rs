use mau::{memo, min, max, sum, and, or};

// 使用 memo 宏优化递归函数
#[memo]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 使用范围宏进行高效的数据处理
fn analyze_data(data: &[i32]) -> (i32, i32, i32, bool, bool) {
    let min_val = min!(|i| data[i], 0..data.len());
    let max_val = max!(|i| data[i], 0..data.len());
    let sum_val = sum!(|i| data[i], 0..data.len());
    
    // 检查是否所有值都大于0
    let all_positive = and!(|i| data[i] > 0, 0..data.len());
    
    // 检查是否有任何值等于0
    let has_zero = or!(|i| data[i] == 0, 0..data.len());
    
    (min_val, max_val, sum_val, all_positive, has_zero)
}

#[test]
fn test_readme_example() {
    // 使用 memo 宏
    assert_eq!(fibonacci(10), 55);
    
    // 使用范围宏
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (min_val, max_val, sum_val, all_positive, has_zero) = analyze_data(&numbers);
    
    assert_eq!(min_val, 1);
    assert_eq!(max_val, 9);
    assert_eq!(sum_val, 31);
    assert_eq!(all_positive, true);
    assert_eq!(has_zero, false);
    
    // 部分范围操作
    let partial_min = min!(|i| numbers[i], 2..6);
    let partial_sum = sum!(|i| numbers[i], 2..6);
    assert_eq!(partial_min, 1);
    assert_eq!(partial_sum, 19);
    
    println!("README 示例测试通过！");
}
