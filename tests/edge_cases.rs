use mau::memo;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn test_edge_cases() {
    println!("=== Testing Edge Cases ===");
    
    // 测试空数据结构
    test_empty_structures();
    
    // 测试单元素数据结构
    test_single_element_structures();
    
    // 测试极大值
    test_large_values();
    
    // 测试重复调用
    test_repeated_calls();
    
    // 测试嵌套递归
    test_nested_recursion();
    
    // 测试类型边界
    test_type_boundaries();
}

fn test_empty_structures() {
    println!("\n--- Empty Structures ---");
    
    // 空 Vec
    let empty_vec1: Vec<i32> = vec![];
    let empty_vec2: Vec<i32> = vec![];
    
    let result1 = process_empty_vec_no_memo(&empty_vec1);
    let result2 = process_empty_vec_no_memo(&empty_vec2);
    let result1_memo = process_empty_vec_memo(&empty_vec1);
    let result2_memo = process_empty_vec_memo(&empty_vec2);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    println!("Empty Vec: {} == {} == {} == {}", result1, result2, result1_memo, result2_memo);
    
    // 空 BTreeSet
    let empty_set1: BTreeSet<i32> = BTreeSet::new();
    let empty_set2: BTreeSet<i32> = BTreeSet::new();
    
    let result1 = process_empty_set_no_memo(&empty_set1);
    let result2 = process_empty_set_no_memo(&empty_set2);
    let result1_memo = process_empty_set_memo(&empty_set1);
    let result2_memo = process_empty_set_memo(&empty_set2);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    println!("Empty BTreeSet: {} == {} == {} == {}", result1, result2, result1_memo, result2_memo);
    
    // 空 BTreeMap
    let empty_map1: BTreeMap<String, i32> = BTreeMap::new();
    let empty_map2: BTreeMap<String, i32> = BTreeMap::new();
    
    let result1 = process_empty_map_no_memo(&empty_map1);
    let result2 = process_empty_map_no_memo(&empty_map2);
    let result1_memo = process_empty_map_memo(&empty_map1);
    let result2_memo = process_empty_map_memo(&empty_map2);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    println!("Empty BTreeMap: {} == {} == {} == {}", result1, result2, result1_memo, result2_memo);
}

fn test_single_element_structures() {
    println!("\n--- Single Element Structures ---");
    
    // 单元素 Vec
    let single_vec1 = vec![42];
    let single_vec2 = vec![42];
    let single_vec3 = vec![43];
    
    let result1 = process_single_vec_no_memo(&single_vec1);
    let result2 = process_single_vec_no_memo(&single_vec2);
    let result3 = process_single_vec_no_memo(&single_vec3);
    
    let result1_memo = process_single_vec_memo(&single_vec1);
    let result2_memo = process_single_vec_memo(&single_vec2);
    let result3_memo = process_single_vec_memo(&single_vec3);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_ne!(result1, result3);
    assert_eq!(result3, result3_memo);
    
    println!("Single Vec: {} == {} == {} != {} == {}", result1, result2, result1_memo, result3, result3_memo);
}

fn test_large_values() {
    println!("\n--- Large Values ---");
    
    // 大数值测试
    let large_vec1 = vec![i32::MAX, i32::MIN, 0, 1, -1];
    let large_vec2 = vec![i32::MAX, i32::MIN, 0, 1, -1];
    let large_vec3 = vec![i32::MAX, i32::MIN, 0, 1, -2]; // 不同值
    
    let result1 = process_large_values_no_memo(&large_vec1);
    let result2 = process_large_values_no_memo(&large_vec2);
    let result3 = process_large_values_no_memo(&large_vec3);
    
    let result1_memo = process_large_values_memo(&large_vec1);
    let result2_memo = process_large_values_memo(&large_vec2);
    let result3_memo = process_large_values_memo(&large_vec3);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_ne!(result1, result3);
    assert_eq!(result3, result3_memo);
    
    println!("Large values: {} == {} == {} != {} == {}", result1, result2, result1_memo, result3, result3_memo);
}

fn test_repeated_calls() {
    println!("\n--- Repeated Calls ---");
    
    let data = vec![1, 2, 3, 4, 5];
    
    // 多次调用相同参数
    let mut results_no_memo = Vec::new();
    let mut results_memo = Vec::new();
    
    for _ in 0..5 {
        results_no_memo.push(repeated_call_no_memo(&data, 3));
        results_memo.push(repeated_call_memo(&data, 3));
    }
    
    // 所有结果应该相同
    for i in 1..results_no_memo.len() {
        assert_eq!(results_no_memo[0], results_no_memo[i]);
        assert_eq!(results_memo[0], results_memo[i]);
    }
    
    assert_eq!(results_no_memo[0], results_memo[0]);
    
    println!("Repeated calls: all {} results are equal", results_no_memo.len());
}

fn test_nested_recursion() {
    println!("\n--- Nested Recursion ---");
    
    let data = vec![1, 2, 3, 4, 5];
    
    let result1 = nested_recursion_no_memo(&data, 0, data.len());
    let result2 = nested_recursion_no_memo(&data, 0, data.len());
    
    let result1_memo = nested_recursion_memo(&data, 0, data.len());
    let result2_memo = nested_recursion_memo(&data, 0, data.len());
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    
    println!("Nested recursion: {} == {} == {} == {}", result1, result2, result1_memo, result2_memo);
}

fn test_type_boundaries() {
    println!("\n--- Type Boundaries ---");
    
    // 测试 usize 边界
    let max_usize = usize::MAX;
    let result1 = boundary_test_no_memo(max_usize);
    let result2 = boundary_test_no_memo(max_usize);
    
    let result1_memo = boundary_test_memo(max_usize);
    let result2_memo = boundary_test_memo(max_usize);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    
    println!("Type boundaries: {} == {} == {} == {}", result1, result2, result1_memo, result2_memo);
    
    // 测试 i32 边界
    let max_i32 = i32::MAX;
    let min_i32 = i32::MIN;
    
    let result1 = i32_boundary_test_no_memo(max_i32, min_i32);
    let result2 = i32_boundary_test_no_memo(max_i32, min_i32);
    
    let result1_memo = i32_boundary_test_memo(max_i32, min_i32);
    let result2_memo = i32_boundary_test_memo(max_i32, min_i32);
    
    assert_eq!(result1, result2);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    
    println!("i32 boundaries: {} == {} == {} == {}", result1, result2, result1_memo, result2_memo);
}

// 处理空 Vec - 不使用 memo
fn process_empty_vec_no_memo(vec: &[i32]) -> i32 {
    vec.len() as i32
}

// 处理空 Vec - 使用 memo
#[memo]
fn process_empty_vec_memo(vec: &[i32]) -> i32 {
    vec.len() as i32
}

// 处理空 BTreeSet - 不使用 memo
fn process_empty_set_no_memo(set: &BTreeSet<i32>) -> i32 {
    set.len() as i32
}

// 处理空 BTreeSet - 使用 memo
#[memo]
fn process_empty_set_memo(set: &BTreeSet<i32>) -> i32 {
    set.len() as i32
}

// 处理空 BTreeMap - 不使用 memo
fn process_empty_map_no_memo(map: &BTreeMap<String, i32>) -> i32 {
    map.len() as i32
}

// 处理空 BTreeMap - 使用 memo
#[memo]
fn process_empty_map_memo(map: &BTreeMap<String, i32>) -> i32 {
    map.len() as i32
}

// 处理单元素 Vec - 不使用 memo
fn process_single_vec_no_memo(vec: &[i32]) -> i32 {
    if vec.is_empty() {
        0
    } else {
        vec[0] * 2
    }
}

// 处理单元素 Vec - 使用 memo
#[memo]
fn process_single_vec_memo(vec: &[i32]) -> i32 {
    if vec.is_empty() {
        0
    } else {
        vec[0] * 2
    }
}

// 处理大数值 - 不使用 memo
fn process_large_values_no_memo(vec: &[i32]) -> i64 {
    vec.iter().map(|&x| x as i64).sum()
}

// 处理大数值 - 使用 memo
#[memo]
fn process_large_values_memo(vec: &[i32]) -> i64 {
    vec.iter().map(|&x| x as i64).sum()
}

// 重复调用测试 - 不使用 memo
fn repeated_call_no_memo(data: &[i32], threshold: i32) -> i32 {
    data.iter().filter(|&&x| x > threshold).sum()
}

// 重复调用测试 - 使用 memo
#[memo]
fn repeated_call_memo(data: &[i32], threshold: i32) -> i32 {
    data.iter().filter(|&&x| x > threshold).sum()
}

// 嵌套递归 - 不使用 memo
fn nested_recursion_no_memo(data: &[i32], start: usize, end: usize) -> i32 {
    if start >= end {
        return 0;
    }
    
    let mid = (start + end) / 2;
    let left = nested_recursion_no_memo(data, start, mid);
    let right = nested_recursion_no_memo(data, mid + 1, end);
    
    left + right + data[mid]
}

// 嵌套递归 - 使用 memo
#[memo]
fn nested_recursion_memo(data: &[i32], start: usize, end: usize) -> i32 {
    if start >= end {
        return 0;
    }
    
    let mid = (start + end) / 2;
    let left = nested_recursion_memo(data, start, mid);
    let right = nested_recursion_memo(data, mid + 1, end);
    
    left + right + data[mid]
}

// 边界测试 - 不使用 memo
fn boundary_test_no_memo(value: usize) -> usize {
    if value == 0 {
        0
    } else {
        value - 1
    }
}

// 边界测试 - 使用 memo
#[memo]
fn boundary_test_memo(value: usize) -> usize {
    if value == 0 {
        0
    } else {
        value - 1
    }
}

// i32 边界测试 - 不使用 memo
fn i32_boundary_test_no_memo(max: i32, min: i32) -> i64 {
    (max as i64) + (min as i64)
}

// i32 边界测试 - 使用 memo
#[memo]
fn i32_boundary_test_memo(max: i32, min: i32) -> i64 {
    (max as i64) + (min as i64)
}
