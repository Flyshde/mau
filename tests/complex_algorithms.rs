use mau::memo;
// use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

#[test]
fn test_complex_algorithms() {
    println!("=== Testing Complex Algorithms ===");
    
    // 测试最长公共子序列
    test_longest_common_subsequence();
    
    // 测试背包问题
    test_knapsack_problem();
    
    // 测试图的最短路径
    test_shortest_path();
    
    // 测试矩阵链乘法
    test_matrix_chain_multiplication();
    
    // 测试编辑距离
    test_edit_distance();
    
    // 测试硬币找零问题
    test_coin_change();
}

fn test_longest_common_subsequence() {
    println!("\n--- Longest Common Subsequence ---");
    
    let s1 = "ABCDGH".to_string();
    let s2 = "AEDFHR".to_string();
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = lcs_no_memo(&s1, &s2, s1.len(), s2.len());
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = lcs_memo(&s1, &s2, s1.len(), s2.len());
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    println!("LCS result: {} == {}", result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Overhead: {:.2}x", overhead);
    }
}

fn test_knapsack_problem() {
    println!("\n--- Knapsack Problem ---");
    
    let weights = vec![10, 20, 30];
    let values = vec![60, 100, 120];
    let capacity = 50;
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = knapsack_no_memo(&weights, &values, capacity, weights.len());
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = knapsack_memo(&weights, &values, capacity, weights.len());
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    println!("Knapsack result: {} == {}", result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Overhead: {:.2}x", overhead);
    }
}

fn test_shortest_path() {
    println!("\n--- Shortest Path (Simple) ---");
    
    let graph = vec![1, 2, 3, 4, 5];
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = simple_path_no_memo(&graph, 0, graph.len() - 1);
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = simple_path_memo(&graph, 0, graph.len() - 1);
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    println!("Shortest path result: {} == {}", result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Overhead: {:.2}x", overhead);
    }
}

fn test_matrix_chain_multiplication() {
    println!("\n--- Matrix Chain Multiplication ---");
    
    let dimensions = vec![1, 2, 3, 4, 5];
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = matrix_chain_no_memo(&dimensions, 1, dimensions.len() - 1);
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = matrix_chain_memo(&dimensions, 1, dimensions.len() - 1);
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    println!("Matrix chain result: {} == {}", result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Overhead: {:.2}x", overhead);
    }
}

fn test_edit_distance() {
    println!("\n--- Edit Distance ---");
    
    let s1 = "kitten".to_string();
    let s2 = "sitting".to_string();
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = edit_distance_no_memo(&s1, &s2, s1.len(), s2.len());
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = edit_distance_memo(&s1, &s2, s1.len(), s2.len());
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    println!("Edit distance result: {} == {}", result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Overhead: {:.2}x", overhead);
    }
}

fn test_coin_change() {
    println!("\n--- Coin Change Problem ---");
    
    let coins = vec![1, 3, 4];
    let amount = 6;
    
    // 不使用 memo 的版本
    let start = Instant::now();
    let result_no_memo = coin_change_no_memo(&coins, amount);
    let time_no_memo = start.elapsed();
    
    // 使用 memo 的版本
    let start = Instant::now();
    let result_memo = coin_change_memo(&coins, amount);
    let time_memo = start.elapsed();
    
    assert_eq!(result_no_memo, result_memo);
    println!("Coin change result: {} == {}", result_no_memo, result_memo);
    
    if time_no_memo > time_memo {
        let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
    } else {
        let overhead = time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64;
        println!("Overhead: {:.2}x", overhead);
    }
}

// 最长公共子序列 - 不使用 memo
fn lcs_no_memo(s1: &String, s2: &String, m: usize, n: usize) -> usize {
    if m == 0 || n == 0 {
        return 0;
    }
    
    if s1.chars().nth(m - 1) == s2.chars().nth(n - 1) {
        1 + lcs_no_memo(s1, s2, m - 1, n - 1)
    } else {
        std::cmp::max(
            lcs_no_memo(s1, s2, m - 1, n),
            lcs_no_memo(s1, s2, m, n - 1)
        )
    }
}

// 最长公共子序列 - 使用 memo
#[memo]
fn lcs_memo(s1: &String, s2: &String, m: usize, n: usize) -> usize {
    if m == 0 || n == 0 {
        return 0;
    }
    
    if s1.chars().nth(m - 1) == s2.chars().nth(n - 1) {
        1 + lcs_memo(s1, s2, m - 1, n - 1)
    } else {
        std::cmp::max(
            lcs_memo(s1, s2, m - 1, n),
            lcs_memo(s1, s2, m, n - 1)
        )
    }
}

// 背包问题 - 不使用 memo
fn knapsack_no_memo(weights: &[i32], values: &[i32], capacity: i32, n: usize) -> i32 {
    if n == 0 || capacity == 0 {
        return 0;
    }
    
    if weights[n - 1] > capacity {
        knapsack_no_memo(weights, values, capacity, n - 1)
    } else {
        std::cmp::max(
            knapsack_no_memo(weights, values, capacity, n - 1),
            values[n - 1] + knapsack_no_memo(weights, values, capacity - weights[n - 1], n - 1)
        )
    }
}

// 背包问题 - 使用 memo
#[memo]
fn knapsack_memo(weights: &[i32], values: &[i32], capacity: i32, n: usize) -> i32 {
    if n == 0 || capacity == 0 {
        return 0;
    }
    
    if weights[n - 1] > capacity {
        knapsack_memo(weights, values, capacity, n - 1)
    } else {
        std::cmp::max(
            knapsack_memo(weights, values, capacity, n - 1),
            values[n - 1] + knapsack_memo(weights, values, capacity - weights[n - 1], n - 1)
        )
    }
}

// 简单路径计算 - 不使用 memo
fn simple_path_no_memo(graph: &[i32], start: usize, end: usize) -> i32 {
    if start >= end {
        return 0;
    }
    
    let mut sum = 0;
    for i in start..=end {
        sum += graph[i];
    }
    
    if end - start > 1 {
        sum += simple_path_no_memo(graph, start + 1, end - 1);
    }
    
    sum
}

// 简单路径计算 - 使用 memo
#[memo]
fn simple_path_memo(graph: &[i32], start: usize, end: usize) -> i32 {
    if start >= end {
        return 0;
    }
    
    let mut sum = 0;
    for i in start..=end {
        sum += graph[i];
    }
    
    if end - start > 1 {
        sum += simple_path_memo(graph, start + 1, end - 1);
    }
    
    sum
}

// 矩阵链乘法 - 不使用 memo
fn matrix_chain_no_memo(dimensions: &[i32], i: usize, j: usize) -> i32 {
    if i == j {
        return 0;
    }
    
    let mut min_cost = i32::MAX;
    for k in i..j {
        let cost = matrix_chain_no_memo(dimensions, i, k) +
                   matrix_chain_no_memo(dimensions, k + 1, j) +
                   dimensions[i - 1] * dimensions[k] * dimensions[j];
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

// 矩阵链乘法 - 使用 memo
#[memo]
fn matrix_chain_memo(dimensions: &[i32], i: usize, j: usize) -> i32 {
    if i == j {
        return 0;
    }
    
    let mut min_cost = i32::MAX;
    for k in i..j {
        let cost = matrix_chain_memo(dimensions, i, k) +
                   matrix_chain_memo(dimensions, k + 1, j) +
                   dimensions[i - 1] * dimensions[k] * dimensions[j];
        if cost < min_cost {
            min_cost = cost;
        }
    }
    min_cost
}

// 编辑距离 - 不使用 memo
fn edit_distance_no_memo(s1: &String, s2: &String, m: usize, n: usize) -> usize {
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    
    if s1.chars().nth(m - 1) == s2.chars().nth(n - 1) {
        edit_distance_no_memo(s1, s2, m - 1, n - 1)
    } else {
        1 + std::cmp::min(
            std::cmp::min(
                edit_distance_no_memo(s1, s2, m - 1, n),
                edit_distance_no_memo(s1, s2, m, n - 1)
            ),
            edit_distance_no_memo(s1, s2, m - 1, n - 1)
        )
    }
}

// 编辑距离 - 使用 memo
#[memo]
fn edit_distance_memo(s1: &String, s2: &String, m: usize, n: usize) -> usize {
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }
    
    if s1.chars().nth(m - 1) == s2.chars().nth(n - 1) {
        edit_distance_memo(s1, s2, m - 1, n - 1)
    } else {
        1 + std::cmp::min(
            std::cmp::min(
                edit_distance_memo(s1, s2, m - 1, n),
                edit_distance_memo(s1, s2, m, n - 1)
            ),
            edit_distance_memo(s1, s2, m - 1, n - 1)
        )
    }
}

// 硬币找零问题 - 不使用 memo
fn coin_change_no_memo(coins: &[i32], amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    if amount < 0 {
        return -1;
    }
    
    let mut min_coins = i32::MAX;
    for &coin in coins {
        let result = coin_change_no_memo(coins, amount - coin);
        if result != -1 {
            min_coins = std::cmp::min(min_coins, 1 + result);
        }
    }
    
    if min_coins == i32::MAX { -1 } else { min_coins }
}

// 硬币找零问题 - 使用 memo
#[memo]
fn coin_change_memo(coins: &[i32], amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }
    if amount < 0 {
        return -1;
    }
    
    let mut min_coins = i32::MAX;
    for &coin in coins {
        let result = coin_change_memo(coins, amount - coin);
        if result != -1 {
            min_coins = std::cmp::min(min_coins, 1 + result);
        }
    }
    
    if min_coins == i32::MAX { -1 } else { min_coins }
}
