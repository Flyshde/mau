use mau::each;

#[test]
fn test_each_basic() {
    let data = vec![3, 1, 4, 1, 5, 9];
    let mut results = Vec::new();
    
    each!(|i| { 
        results.push(data[i]); 
    }, 0..data.len());
    
    assert_eq!(results, vec![3, 1, 4, 1, 5, 9]);
}

#[test]
fn test_each_partial_range() {
    let data = vec![3, 1, 4, 1, 5, 9];
    let mut results = Vec::new();
    
    each!(|i| { 
        results.push(data[i]); 
    }, 2..5);
    
    assert_eq!(results, vec![4, 1, 5]);
}

#[test]
fn test_each_with_closure_body() {
    let data = vec![10, 20, 30];
    let mut sum = 0;
    
    each!(|i| {
        sum += data[i];
    }, 0..data.len());
    
    assert_eq!(sum, 60);
}

#[test]
fn test_each_equivalent_to_for_loop() {
    let data = vec![3, 1, 4, 1, 5, 9];
    let mut results_with_each = Vec::new();
    let mut results_with_for = Vec::new();
    
    // 使用 each! 宏
    each!(|i| {
        results_with_each.push(data[i]);
    }, 0..data.len());
    
    // 使用传统的 for 循环（等价写法）
    for i in 0..data.len() {
        results_with_for.push(data[i]);
    }
    
    // 验证两种方式结果相同
    assert_eq!(results_with_each, results_with_for);
}

#[test]
fn test_each_with_range_to() {
    let data = vec![3, 1, 4, 1, 5, 9];
    let mut results = Vec::new();
    
    each!(|i| {
        results.push(data[i]);
    }, 1..=4);  // 包含结束边界
    
    assert_eq!(results, vec![1, 4, 1, 5]);
}

