use mau::{min, max, sum, and, or};

#[test]
fn test_copy_types_auto_deref() {
    // 测试Copy类型的自动解引用
    let numbers: Vec<usize> = vec![1, 2, 3, 4, 5];
    
    // 迭代器语法应该自动解引用Copy类型
    let min_val = min!(|x: usize| x, numbers.iter());
    assert_eq!(min_val, 1);
    
    let max_val = max!(|x: usize| x, numbers.iter());
    assert_eq!(max_val, 5);
    
    let sum_val = sum!(|x: usize| x, numbers.iter());
    assert_eq!(sum_val, 15);
    
    // 浮点数测试
    let floats = vec![3.5, 1.2, 4.8, 1.1, 5.9];
    let min_float = min!(|f| f, floats.iter());
    assert_eq!(min_float, 1.1);
    
    // 布尔值测试
    let bools = vec![true, false, true, true];
    let and_result = and!(|b| b, bools.iter());
    assert_eq!(and_result, false);
    
    let or_result = or!(|b| b, bools.iter());
    assert_eq!(or_result, true);
}

#[test]
fn test_string_slices() {
    // 测试字符串切片
    let words = vec!["apple", "banana", "cherry"];
    let min_len = min!(|s: &str| s.len(), words.iter());
    assert_eq!(min_len, 5);
    
    let min_word = min!(|s: &str| s.len(), words.iter());
    assert_eq!(min_word, 5);
}

#[test]
fn test_mixed_types() {
    // 测试混合类型
    let data = vec![1, 2, 3, 4, 5];
    
    // 范围语法（索引值，不需要解引用）
    let range_result = min!(|i| data[i], 0..data.len());
    assert_eq!(range_result, 1);
    
    // 迭代器语法（自动解引用Copy类型）
    let iter_result = min!(|x| x, data.iter());
    assert_eq!(iter_result, 1);
    
    // 简写语法（内部转换为范围语法）
    let shorthand_result = min!(data);
    assert_eq!(shorthand_result, 1);
    
    // 多参数语法（内部转换为范围语法）
    let multi_result = min!(1, 2, 3, 4, 5);
    assert_eq!(multi_result, 1);
}

#[test]
fn test_complex_expressions() {
    // 测试复杂表达式
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 迭代器语法中的复杂表达式
    let doubled_sum = sum!(|x| x * 2, numbers.iter());
    assert_eq!(doubled_sum, 30); // (1+2+3+4+5)*2 = 30
    
    // 范围语法中的复杂表达式
    let range_doubled_sum = sum!(|i| numbers[i] * 2, 0..numbers.len());
    assert_eq!(range_doubled_sum, 30);
    
    // 字符串长度计算
    let words = vec!["apple", "banana", "cherry"];
    let min_length = min!(|s: &str| s.len(), words.iter());
    assert_eq!(min_length, 5);
}

#[test]
fn test_edge_cases() {
    // 测试边界情况
    let single = vec![42];
    let single_min = min!(|x| x, single.iter());
    assert_eq!(single_min, 42);
}

#[test]
fn test_different_iterator_types() {
    // 测试不同的迭代器类型
    let array = [10, 20, 30, 40, 50];
    let array_max = max!(|x| x, array.iter());
    assert_eq!(array_max, 50);
    
    let vec_data = vec![1, 2, 3, 4, 5];
    let vec_max = max!(|x| x, vec_data.iter());
    assert_eq!(vec_max, 5);
    
    // 测试VecDeque
    use std::collections::VecDeque;
    let deque: VecDeque<i32> = vec![100, 200, 300].into();
    let deque_max = max!(|x| x, deque.iter());
    assert_eq!(deque_max, 300);
}