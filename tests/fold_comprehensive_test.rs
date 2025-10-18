use mau::fold;

// ========== 基础数值测试 ==========

#[test]
fn test_fold_negative_numbers() {
    let data = vec![-5, -3, -8, -1, -4];
    let sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum, -21);
    
    let min = fold!(i32::MAX, |i| data[i], 0..data.len(), |acc, val| {
        if val < acc { val } else { acc }
    });
    assert_eq!(min, -8);
}

#[test]
fn test_fold_floating_point() {
    let data = vec![1.5, 2.5, 3.5, 4.5];
    let sum = fold!(0.0, |i| data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum, 12.0);
    
    let product = fold!(1.0, |i| data[i], 0..data.len(), |acc, val| acc * val);
    assert_eq!(product, 59.0625); // 1.5 * 2.5 * 3.5 * 4.5
}

#[test]
fn test_fold_mixed_positive_negative() {
    let data = vec![10, -5, 3, -2, 7];
    let sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum, 13);
}

// ========== 复杂数据结构测试 ==========

#[test]
fn test_fold_nested_vectors() {
    let data = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    // 展平并求和
    let sum = fold!(0, |i| &data[i], 0..data.len(), |acc, val: &Vec<i32>| {
        acc + val.iter().sum::<i32>()
    });
    assert_eq!(sum, 45);
}

#[test]
fn test_fold_string_operations() {
    let words: Vec<&str> = vec!["Rust", "is", "awesome"];
    
    // 构建句子
    let sentence = fold!(String::new(), |i| words[i], 0..words.len(), |mut acc: String, val: &str| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(val);
        acc
    });
    assert_eq!(sentence, "Rust is awesome");
    
    // 计算总字符数
    let total_chars = fold!(0_usize, |i: usize| words[i].len(), 0..words.len(), |acc: usize, len: usize| acc + len);
    assert_eq!(total_chars, 13); // "Rust" + "is" + "awesome"
}

#[test]
fn test_fold_hashmap_building() {
    use std::collections::HashMap;
    
    let keys = vec![String::from("a"), String::from("b"), String::from("c")];
    let values = vec![1_i32, 2, 3];
    
    let map = fold!(HashMap::<String, i32>::new(), |i: usize| (keys[i].clone(), values[i]), 0..keys.len(), 
        |mut acc: HashMap<String, i32>, (k, v): (String, i32)| {
            acc.insert(k, v);
            acc
        }
    );
    
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
    assert_eq!(map.get("c"), Some(&3));
}

// ========== 条件和过滤测试 ==========

#[test]
fn test_fold_conditional_accumulation() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 只累加偶数
    let even_sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| {
        if val % 2 == 0 {
            acc + val
        } else {
            acc
        }
    });
    assert_eq!(even_sum, 30); // 2 + 4 + 6 + 8 + 10
    
    // 只累加奇数
    let odd_sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| {
        if val % 2 == 1 {
            acc + val
        } else {
            acc
        }
    });
    assert_eq!(odd_sum, 25); // 1 + 3 + 5 + 7 + 9
}

#[test]
fn test_fold_with_threshold() {
    let data = vec![1, 5, 3, 8, 2, 9, 4];
    
    // 计数大于5的元素
    let count_gt_5 = fold!(0, |i| data[i], 0..data.len(), |acc, val| {
        if val > 5 {
            acc + 1
        } else {
            acc
        }
    });
    assert_eq!(count_gt_5, 2); // 8, 9
}

// ========== 统计和分析测试 ==========

#[test]
fn test_fold_statistics() {
    let data = vec![10, 20, 30, 40, 50];
    
    // 计算平均值（使用元组累加器）
    let (sum, count) = fold!((0, 0), |i| data[i], 0..data.len(), |(s, c), val| {
        (s + val, c + 1)
    });
    let avg = sum / count;
    assert_eq!(avg, 30);
}

#[test]
fn test_fold_min_max_together() {
    let data = vec![5, 2, 8, 1, 9, 3];
    
    // 同时找最小值和最大值
    let (min, max) = fold!((i32::MAX, i32::MIN), |i| data[i], 0..data.len(), |(min_val, max_val), val| {
        (
            if val < min_val { val } else { min_val },
            if val > max_val { val } else { max_val }
        )
    });
    
    assert_eq!(min, 1);
    assert_eq!(max, 9);
}

// ========== 范围和迭代器测试 ==========

#[test]
fn test_fold_partial_range() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 只对中间部分求和
    let sum = fold!(0, |i| data[i], 3..7, |acc, val| acc + val);
    assert_eq!(sum, 22); // 4 + 5 + 6 + 7
}

#[test]
fn test_fold_inclusive_range() {
    let data = vec![10, 20, 30, 40, 50];
    
    let sum_exclusive = fold!(0, |i| data[i], 1..3, |acc, val| acc + val);
    assert_eq!(sum_exclusive, 50); // 20 + 30
    
    let sum_inclusive = fold!(0, |i| data[i], 1..=3, |acc, val| acc + val);
    assert_eq!(sum_inclusive, 90); // 20 + 30 + 40
}

#[test]
fn test_fold_empty_range() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 空范围应该返回初始值
    let result = fold!(42, |i| data[i], 2..2, |acc, val| acc + val);
    assert_eq!(result, 42);
}

#[test]
fn test_fold_with_iterator() {
    let data = vec![1, 2, 3, 4, 5];
    let indices = vec![0_usize, 2, 4];
    
    // 使用迭代器而不是范围
    let sum = fold!(0, |i: &usize| data[*i], &indices, |acc, val| acc + val);
    assert_eq!(sum, 9); // data[0] + data[2] + data[4] = 1 + 3 + 5
}

// ========== 类型转换测试 ==========

#[test]
fn test_fold_type_conversion() {
    let integers = vec![1, 2, 3, 4, 5];
    
    // 整数转浮点数累加
    let float_sum = fold!(0.0, |i| integers[i] as f64, 0..integers.len(), |acc, val| acc + val);
    assert_eq!(float_sum, 15.0);
}

#[test]
fn test_fold_accumulator_different_type() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 累加器类型为 String，元素类型为 i32
    let string_result = fold!(String::new(), |i| numbers[i], 0..numbers.len(), |mut acc: String, val: i32| {
        if !acc.is_empty() {
            acc.push(',');
        }
        acc.push_str(&val.to_string());
        acc
    });
    assert_eq!(string_result, "1,2,3,4,5");
}

// ========== 复杂计算测试 ==========

#[test]
fn test_fold_factorial() {
    let data = vec![1, 2, 3, 4, 5];
    let factorial = fold!(1, |i| data[i], 0..data.len(), |acc, val| acc * val);
    assert_eq!(factorial, 120);
}

#[test]
fn test_fold_power_sum() {
    let data = vec![1, 2, 3, 4];
    
    // 计算平方和
    let sum_of_squares = fold!(0, |i| data[i] * data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum_of_squares, 30); // 1 + 4 + 9 + 16
}

#[test]
fn test_fold_reverse_string() {
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    
    let reversed = fold!(String::new(), |i| chars[chars.len() - 1 - i], 0..chars.len(), |mut acc: String, c: char| {
        acc.push(c);
        acc
    });
    assert_eq!(reversed, "olleH");
}

// ========== 边界情况测试 ==========

#[test]
fn test_fold_single_element() {
    let data = vec![42];
    let sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum, 42);
}

#[test]
fn test_fold_large_initial_value() {
    let data = vec![1, 2, 3];
    let result = fold!(1000, |i| data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(result, 1006);
}

#[test]
fn test_fold_zero_initial() {
    let data = vec![5, 10, 15];
    let product = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc * val);
    assert_eq!(product, 0); // 0 * anything = 0
}

// ========== 实际应用场景测试 ==========

#[test]
fn test_fold_word_count() {
    let text: Vec<&str> = vec!["hello world", "rust is great", "fold is powerful"];
    
    let word_count = fold!(0_usize, |i: usize| text[i].split_whitespace().count(), 0..text.len(), |acc: usize, count: usize| acc + count);
    assert_eq!(word_count, 8); // 2 + 3 + 3
}

#[test]
fn test_fold_matrix_sum() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9]
    ];
    
    let total = fold!(0, |i| &matrix[i], 0..matrix.len(), |acc, row: &Vec<i32>| {
        acc + row.iter().sum::<i32>()
    });
    assert_eq!(total, 45);
}

#[test]
fn test_fold_running_average() {
    let data = vec![10.0, 20.0, 30.0, 40.0];
    
    // 计算总和和数量，然后计算平均值
    let (sum, count) = fold!((0.0, 0), |i| (data[i], 1), 0..data.len(), |(sum_acc, cnt_acc): (f64, usize), (val, one): (f64, usize)| {
        (sum_acc + val, cnt_acc + one)
    });
    let avg = sum / (count as f64);
    assert_eq!(avg, 25.0); // (10 + 20 + 30 + 40) / 4
}

#[test]
fn test_fold_boolean_all_any() {
    let data1 = vec![true, true, true];
    let all_true = fold!(true, |i| data1[i], 0..data1.len(), |acc, val| acc && val);
    assert_eq!(all_true, true);
    
    let data2 = vec![false, false, true];
    let any_true = fold!(false, |i| data2[i], 0..data2.len(), |acc, val| acc || val);
    assert_eq!(any_true, true);
}

