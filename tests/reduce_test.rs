use mau::reduce;

#[test]
fn test_reduce_max() {
    // 测试找最大值
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let max_val = reduce!(|i| data[i], 0..data.len(), |a, b| if a > b { a } else { b });
    assert_eq!(max_val, 9);
}

#[test]
fn test_reduce_min() {
    // 测试找最小值
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let min_val = reduce!(|i| data[i], 0..data.len(), |a, b| if a < b { a } else { b });
    assert_eq!(min_val, 1);
}

#[test]
fn test_reduce_sum() {
    // 测试求和
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sum_val = reduce!(|i| data[i], 0..data.len(), |a, b| a + b);
    assert_eq!(sum_val, 31); // 3+1+4+1+5+9+2+6 = 31
}

#[test]
fn test_reduce_product() {
    // 测试求积
    let data = vec![2, 3, 4];
    let product = reduce!(|i| data[i], 0..data.len(), |a, b| a * b);
    assert_eq!(product, 24); // 2*3*4 = 24
}

#[test]
fn test_reduce_partial_range() {
    // 测试部分范围
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let partial_max = reduce!(|i| data[i], 2..6, |a, b| if a > b { a } else { b });
    assert_eq!(partial_max, 9); // 只检查索引2到5: [4, 1, 5, 9]
}

#[test]
fn test_reduce_inclusive_range() {
    // 测试包含范围
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let inclusive_max = reduce!(|i| data[i], 2..=5, |a, b| if a > b { a } else { b });
    assert_eq!(inclusive_max, 9); // 检查索引2到5: [4, 1, 5, 9]
}

#[test]
fn test_reduce_string_length() {
    // 测试字符串长度比较
    let words: Vec<&str> = vec!["apple", "banana", "cherry", "date"];
    let max_length = reduce!(|i: usize| words[i].len(), 0..words.len(), |a, b| if a > b { a } else { b });
    assert_eq!(max_length, 6); // "banana" has 6 characters
}

#[test]
fn test_reduce_float_arrays() {
    // 测试浮点数组
    let floats = vec![3.5, 1.2, 4.8, 1.1, 5.9, 2.3];
    let max_float = reduce!(|i| floats[i], 0..floats.len(), |a, b| if a > b { a } else { b });
    assert_eq!(max_float, 5.9);
    
    let sum_float = reduce!(|i| floats[i], 0..floats.len(), |a, b| a + b);
    assert_eq!(sum_float, 18.8); // 3.5+1.2+4.8+1.1+5.9+2.3 = 18.8
}

#[test]
fn test_reduce_boolean_arrays() {
    // 测试布尔数组
    let bools = vec![true, true, false, true];
    let and_result = reduce!(|i| bools[i], 0..bools.len(), |a, b| a && b);
    assert_eq!(and_result, false); // true && true && false && true = false
    
    let or_result = reduce!(|i| bools[i], 0..bools.len(), |a, b| a || b);
    assert_eq!(or_result, true); // true || true || false || true = true
}

#[test]
fn test_reduce_complex_expression() {
    // 测试复杂表达式
    let data = vec![1, 2, 3, 4, 5];
    let max_doubled = reduce!(|i| data[i] * 2, 0..data.len(), |a, b| if a > b { a } else { b });
    assert_eq!(max_doubled, 10); // max(2, 4, 6, 8, 10) = 10
    
    let sum_squared = reduce!(|i| data[i] * data[i], 0..data.len(), |a, b| a + b);
    assert_eq!(sum_squared, 55); // 1+4+9+16+25 = 55
}

#[test]
fn test_reduce_single_element() {
    // 测试单元素数组
    let data = vec![42];
    let result = reduce!(|i| data[i], 0..data.len(), |a, b| if a > b { a } else { b });
    assert_eq!(result, 42);
}

#[test]
fn test_reduce_empty_range() {
    // 测试空范围（应该panic）
    let data = vec![1, 2, 3];
    let result = std::panic::catch_unwind(|| {
        reduce!(|i| data[i], 2..2, |a, b| if a > b { a } else { b })
    });
    assert!(result.is_err());
}

#[test]
fn test_reduce_custom_operation() {
    // 测试自定义操作
    let data = vec![1, 2, 3, 4, 5];
    
    // 找最大偶数
      let max_even = reduce!(|i| data[i], 0..data.len(), |a, b| {
        if a % 2 == 0 && b % 2 == 0 {
            if a > b { a } else { b }
        } else if a % 2 == 0 {
            a
        } else if b % 2 == 0 {
            b
        } else {
            a
        }
    });
    assert_eq!(max_even, 4);
    
    // 字符串连接
    let words: Vec<&str> = vec!["hello", "world", "rust"];
    let concatenated = reduce!(|i: usize| words[i].to_string(), 0..words.len(), |a, b| format!("{} {}", a, b));
    assert_eq!(concatenated, "hello world rust");
}
