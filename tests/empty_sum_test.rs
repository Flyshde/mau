use mau::sum;

#[test]
fn test_sum_empty_i32() {
    let empty: Vec<i32> = vec![];
    let result = sum!(empty);
    assert_eq!(result, 0);
}

#[test]
fn test_sum_empty_u64() {
    let empty: Vec<u64> = vec![];
    let result = sum!(empty);
    assert_eq!(result, 0);
}

#[test]
fn test_sum_empty_f64() {
    let empty: Vec<f64> = vec![];
    let result = sum!(empty);
    assert_eq!(result, 0.0);
}

#[test]
fn test_sum_empty_range() {
    let data = vec![1, 2, 3];
    // 空范围
    let result = sum!(|i| data[i], 0..0);
    assert_eq!(result, 0);
}

#[test]
fn test_sum_non_empty() {
    let data = vec![1, 2, 3, 4, 5];
    let result = sum!(data);
    assert_eq!(result, 15);
}

#[test]
fn test_sum_mixed_with_empty() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 非空范围
    let result1 = sum!(|i| data[i], 0..5);
    assert_eq!(result1, 15);
    
    // 空范围应该返回 0
    let result2 = sum!(|i| data[i], 0..0);
    assert_eq!(result2, 0);
    
    // 部分范围
    let result3 = sum!(|i| data[i], 2..4);
    assert_eq!(result3, 7);  // 3 + 4
}

