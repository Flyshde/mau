use mau::fold;

#[test]
fn test_fold_sum() {
    let data = vec![1, 2, 3, 4, 5];
    let sum = fold!(0, |i| data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum, 15);
}

#[test]
fn test_fold_product() {
    let data = vec![1, 2, 3, 4, 5];
    let product = fold!(1, |i| data[i], 0..data.len(), |acc, val| acc * val);
    assert_eq!(product, 120);
}

#[test]
fn test_fold_empty_range() {
    let data = vec![1, 2, 3, 4, 5];
    // 空范围应该返回初始值
    let result = fold!(100, |i| data[i], 0..0, |acc, val| acc + val);
    assert_eq!(result, 100);
}

#[test]
fn test_fold_partial_range() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // 只对索引 2 到 5 的元素求和：[4, 1, 5, 9]
    let sum = fold!(0, |i| data[i], 2..6, |acc, val| acc + val);
    assert_eq!(sum, 19);
}

#[test]
fn test_fold_inclusive_range() {
    let data = vec![1, 2, 3, 4, 5];
    // 使用闭区间 0..=4
    let sum = fold!(0, |i| data[i], 0..=4, |acc, val| acc + val);
    assert_eq!(sum, 15);
}

#[test]
fn test_fold_string_building() {
    let words = vec!["Hello", "World", "Rust"];
    let sentence = fold!(String::new(), |i| words[i], 0..words.len(), |mut acc: String, val| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(val);
        acc
    });
    assert_eq!(sentence, "Hello World Rust");
}

#[test]
fn test_fold_count_elements() {
    let data = vec![10, 20, 30, 40, 50];
    // 计数：不管元素值是什么，每次加1
    let count = fold!(0, |i| data[i], 0..data.len(), |acc, _val| acc + 1);
    assert_eq!(count, 5);
}

#[test]
fn test_fold_max_with_init() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    // 使用 i32::MIN 作为初始值找最大值
    let max = fold!(i32::MIN, |i| data[i], 0..data.len(), |acc, val| {
        if val > acc { val } else { acc }
    });
    assert_eq!(max, 9);
}

#[test]
fn test_fold_vec_collection() {
    let data = vec![1, 2, 3, 4, 5];
    // 收集偶数到一个新向量
    let evens = fold!(Vec::<i32>::new(), |i| data[i], 0..data.len(), |mut acc: Vec<i32>, val| {
        if val % 2 == 0 {
            acc.push(val);
        }
        acc
    });
    assert_eq!(evens, vec![2, 4]);
}

#[test]
fn test_fold_different_types() {
    let numbers = vec![1, 2, 3, 4, 5];
    // 累加器类型(f64)与元素类型(i32)不同
    let avg_sum = fold!(0.0, |i| numbers[i], 0..numbers.len(), |acc, val| {
        acc + val as f64
    });
    assert_eq!(avg_sum, 15.0);
}

#[test]
fn test_fold_complex_expression() {
    let data = vec![1, 2, 3, 4, 5];
    // 计算平方和
    let sum_of_squares = fold!(0, |i| data[i] * data[i], 0..data.len(), |acc, val| acc + val);
    assert_eq!(sum_of_squares, 55); // 1 + 4 + 9 + 16 + 25
}

#[test]
fn test_fold_with_tuples() {
    let data = vec![1, 2, 3, 4, 5];
    // 同时计算和与积
    let (sum, product) = fold!((0, 1), |i| data[i], 0..data.len(), |(s, p), val| {
        (s + val, p * val)
    });
    assert_eq!(sum, 15);
    assert_eq!(product, 120);
}

#[test]
fn test_fold_boolean_operations() {
    let data = vec![true, true, true, true];
    // 使用 fold 实现 and 操作
    let all_true = fold!(true, |i| data[i], 0..data.len(), |acc, val| acc && val);
    assert_eq!(all_true, true);
    
    let data2 = vec![true, true, false, true];
    let has_false = fold!(true, |i| data2[i], 0..data2.len(), |acc, val| acc && val);
    assert_eq!(has_false, false);
}

#[test]
fn test_fold_iterator_like() {
    // 使用迭代器而不是范围
    let data = vec![1, 2, 3, 4, 5];
    let indices = vec![0_usize, 2, 4]; // 只处理索引 0, 2, 4
    let sum = fold!(0, |i: &usize| data[*i], &indices, |acc, val| acc + val);
    assert_eq!(sum, 9); // data[0] + data[2] + data[4] = 1 + 3 + 5
}

