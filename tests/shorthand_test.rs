use mau::{min, max, sum, and, or};

#[test]
fn test_shorthand_syntax() {
    // 测试简写语法：sum!(d) 等价于 sum!(|i| d[i], 0..d.len())
    let d = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // 简写语法
    let min_val = min!(d);
    let max_val = max!(d);
    let sum_val = sum!(d);
    
    // 完整语法
    let min_val_full = min!(|i| d[i], 0..d.len());
    let max_val_full = max!(|i| d[i], 0..d.len());
    let sum_val_full = sum!(|i| d[i], 0..d.len());
    
    // 验证结果相同
    assert_eq!(min_val, min_val_full);
    assert_eq!(max_val, max_val_full);
    assert_eq!(sum_val, sum_val_full);
    
    // 验证具体值
    assert_eq!(min_val, 1);
    assert_eq!(max_val, 9);
    assert_eq!(sum_val, 31); // 3+1+4+1+5+9+2+6 = 31
}

#[test]
fn test_shorthand_boolean_arrays() {
    // 测试布尔数组的简写语法
    let bools = vec![true, true, false, true];
    
    // 简写语法
    let and_val = and!(bools);
    let or_val = or!(bools);
    
    // 完整语法
    let and_val_full = and!(|i| bools[i], 0..bools.len());
    let or_val_full = or!(|i| bools[i], 0..bools.len());
    
    // 验证结果相同
    assert_eq!(and_val, and_val_full);
    assert_eq!(or_val, or_val_full);
    
    // 验证具体值
    assert_eq!(and_val, false); // true && true && false && true = false
    assert_eq!(or_val, true);   // true || true || false || true = true
}

#[test]
fn test_shorthand_float_arrays() {
    // 测试浮点数组的简写语法
    let floats = vec![3.5, 1.2, 4.8, 1.1, 5.9, 2.3];
    
    let min_float = min!(floats);
    let max_float = max!(floats);
    let sum_float = sum!(floats);
    
    assert_eq!(min_float, 1.1);
    assert_eq!(max_float, 5.9);
    assert_eq!(sum_float, 18.8); // 3.5+1.2+4.8+1.1+5.9+2.3 = 18.8
}

#[test]
fn test_shorthand_string_arrays() {
    // 测试字符串数组的简写语法（按长度比较）
    let words: Vec<&str> = vec!["apple", "banana", "cherry", "date"];
    
    // 对于字符串，我们需要使用范围语法来按长度比较
    let min_length = min!(|i: usize| words[i].len(), 0..words.len());
    let max_length = max!(|i: usize| words[i].len(), 0..words.len());
    let sum_length = sum!(|i: usize| words[i].len(), 0..words.len());
    
    assert_eq!(min_length, 4); // "date" has 4 characters
    assert_eq!(max_length, 6); // "banana" has 6 characters
    assert_eq!(sum_length, 21); // 5+6+6+4 = 21
}

#[test]
fn test_shorthand_empty_array() {
    // 测试空数组的情况 - min!
    let empty_min: Vec<i32> = vec![];
    let min_result = min!(empty_min);
    assert_eq!(min_result, i32::MAX);
    
    // 测试空数组的情况 - max!
    let empty_max: Vec<i32> = vec![];
    let max_result = max!(empty_max);
    assert_eq!(max_result, i32::MIN);
    
    // 测试空数组的情况 - sum!
    let empty_sum: Vec<i32> = vec![];
    let sum_result = sum!(empty_sum);
    assert_eq!(sum_result, 0);
}

#[test]
fn test_shorthand_single_element() {
    // 测试单元素数组
    let single = vec![42];
    
    let min_val = min!(single);
    let max_val = max!(single);
    let sum_val = sum!(single);
    
    let bools = vec![true];
    let and_val = and!(bools);
    
    let bools2 = vec![false];
    let or_val = or!(bools2);
    
    assert_eq!(min_val, 42);
    assert_eq!(max_val, 42);
    assert_eq!(sum_val, 42);
    assert_eq!(and_val, true);
    assert_eq!(or_val, false);
}

#[test]
fn test_shorthand_vs_multi_args() {
    // 测试简写语法与多参数语法的区别
    let d = vec![3, 1, 4];
    
    // 简写语法：对整个数组操作
    let sum_array = sum!(d);
    
    // 多参数语法：对多个独立值操作
    let sum_multi = sum!(3, 1, 4);
    
    // 结果应该相同
    assert_eq!(sum_array, sum_multi);
    assert_eq!(sum_array, 8);
}

#[test]
fn test_shorthand_with_expressions() {
    // 测试简写语法与表达式的结合
    let d = vec![1, 2, 3, 4, 5];
    
    // 简写语法直接对数组操作
    let sum_direct = sum!(d);
    
    // 范围语法对变换后的值操作
    let sum_doubled = sum!(|i| d[i] * 2, 0..d.len());
    
    assert_eq!(sum_direct, 15); // 1+2+3+4+5 = 15
    assert_eq!(sum_doubled, 30); // (1+2+3+4+5)*2 = 30
}

#[test]
fn test_shorthand_array_literals() {
    // 测试数组字面量的简写语法
    let min_val = min!([1, 3, 2, 4]);
    let max_val = max!([1, 3, 2, 4]);
    let sum_val = sum!([1, 3, 2, 4]);
    
    assert_eq!(min_val, 1);
    assert_eq!(max_val, 4);
    assert_eq!(sum_val, 10); // 1+3+2+4 = 10
}

#[test]
fn test_shorthand_vec_macro() {
    // 测试vec!宏的简写语法
    let numbers = vec![5, 2, 8, 1, 9];
    let min_val = min!(numbers);
    let max_val = max!(numbers);
    let sum_val = sum!(numbers);
    
    assert_eq!(min_val, 1);
    assert_eq!(max_val, 9);
    assert_eq!(sum_val, 25); // 5+2+8+1+9 = 25
}

#[test]
fn test_shorthand_mixed_types() {
    // 测试混合类型的简写语法
    let ints = vec![1, 2, 3];
    let floats = vec![1.5, 2.5, 3.5];
    let bools = vec![true, false, true];
    
    // 整数数组
    let min_int = min!(ints);
    let sum_int = sum!(ints);
    
    // 浮点数组
    let min_float = min!(floats);
    let sum_float = sum!(floats);
    
    // 布尔数组
    let and_val = and!(bools);
    let or_val = or!(bools);
    
    assert_eq!(min_int, 1);
    assert_eq!(sum_int, 6);
    assert_eq!(min_float, 1.5);
    assert_eq!(sum_float, 7.5);
    assert_eq!(and_val, false);
    assert_eq!(or_val, true);
}
