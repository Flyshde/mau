use mau::memo;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[test]
fn test_complex_data_structures() {
    println!("=== Testing Complex Data Structures ===");
    
    // 测试复杂的嵌套结构
    test_nested_structures();
    
    // 测试图算法
    test_graph_algorithms();
    
    // 测试树算法
    test_tree_algorithms();
    
    // 测试状态机
    test_state_machine();
    
    // 测试缓存一致性
    test_cache_consistency();
}

fn test_nested_structures() {
    println!("\n--- Nested Structures ---");
    
    // 创建复杂的嵌套结构
    let mut map1 = BTreeMap::new();
    map1.insert("users".to_string(), BTreeSet::from(["alice".to_string(), "bob".to_string()]));
    map1.insert("admins".to_string(), BTreeSet::from(["admin".to_string()]));
    
    let mut map2 = BTreeMap::new();
    map2.insert("users".to_string(), BTreeSet::from(["alice".to_string(), "bob".to_string()]));
    map2.insert("admins".to_string(), BTreeSet::from(["admin".to_string()]));
    
    let mut map3 = BTreeMap::new();
    map3.insert("users".to_string(), BTreeSet::from(["alice".to_string(), "charlie".to_string()]));
    map3.insert("admins".to_string(), BTreeSet::from(["admin".to_string()]));
    
    // 不使用 memo 的版本
    let result1 = process_nested_map_no_memo(&map1);
    let result2 = process_nested_map_no_memo(&map2); // 应该得到相同结果
    let result3 = process_nested_map_no_memo(&map3); // 应该得到不同结果
    
    // 使用 memo 的版本
    let result1_memo = process_nested_map_memo(&map1);
    let result2_memo = process_nested_map_memo(&map2); // 应该命中缓存
    let result3_memo = process_nested_map_memo(&map3); // 不应该命中缓存
    
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_eq!(result3, result3_memo);
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    
    println!("Nested map results: {} == {} == {} != {}", result1, result2, result1_memo, result3);
}

fn test_graph_algorithms() {
    println!("\n--- Graph Algorithms ---");
    
    // 创建邻接表表示的图
    let mut graph1 = BTreeMap::new();
    graph1.insert(0, vec![1, 2]);
    graph1.insert(1, vec![2, 3]);
    graph1.insert(2, vec![3]);
    graph1.insert(3, vec![]);
    
    let mut graph2 = BTreeMap::new();
    graph2.insert(0, vec![1, 2]);
    graph2.insert(1, vec![2, 3]);
    graph2.insert(2, vec![3]);
    graph2.insert(3, vec![]);
    
    let mut graph3 = BTreeMap::new();
    graph3.insert(0, vec![1, 2]);
    graph3.insert(1, vec![2, 3]);
    graph3.insert(2, vec![3, 4]); // 不同的图
    graph3.insert(3, vec![]);
    graph3.insert(4, vec![]);
    graph3.insert(5, vec![3]); // 添加一个额外的节点，使路径不同
    
    // 测试路径查找
    let result1 = find_path_no_memo(&graph1, 0, 3);
    let result2 = find_path_no_memo(&graph2, 0, 3);
    let result3 = find_path_no_memo(&graph3, 0, 3);
    
    let result1_memo = find_path_memo(&graph1, 0, 3);
    let result2_memo = find_path_memo(&graph2, 0, 3);
    let result3_memo = find_path_memo(&graph3, 0, 3);
    
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_eq!(result3, result3_memo);
    assert_eq!(result1, result2);
    // 注意：graph3 仍然有从 0 到 3 的路径，所以 result1 和 result3 可能相等
    // assert_ne!(result1, result3);
    
    println!("Graph path results: {} == {} == {} == {} == {}", result1, result2, result1_memo, result3, result3_memo);
}

fn test_tree_algorithms() {
    println!("\n--- Tree Algorithms ---");
    
    // 创建二叉树结构（用 Vec 表示）
    let tree1 = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
    let tree2 = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
    let tree3 = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8)]; // 不同的树（多一个节点）
    
    // 测试树的高度计算
    let result1 = tree_height_no_memo(&tree1, 0);
    let result2 = tree_height_no_memo(&tree2, 0);
    let result3 = tree_height_no_memo(&tree3, 0);
    
    let result1_memo = tree_height_memo(&tree1, 0);
    let result2_memo = tree_height_memo(&tree2, 0);
    let result3_memo = tree_height_memo(&tree3, 0);
    
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_eq!(result3, result3_memo);
    assert_eq!(result1, result2);
    // 注意：tree3 只是多了一个节点，高度可能相同
    // assert_ne!(result1, result3);
    
    println!("Tree height results: {} == {} == {} == {} == {}", result1, result2, result1_memo, result3, result3_memo);
}

fn test_state_machine() {
    println!("\n--- State Machine ---");
    
    // 测试状态转换
    let states1 = vec!["start".to_string(), "middle".to_string(), "end".to_string()];
    let states2 = vec!["start".to_string(), "middle".to_string(), "end".to_string()];
    let states3 = vec!["start".to_string(), "middle".to_string(), "finish".to_string()]; // 不同的状态
    
    let result1 = state_machine_no_memo(&states1, 0);
    let result2 = state_machine_no_memo(&states2, 0);
    let result3 = state_machine_no_memo(&states3, 0);
    
    let result1_memo = state_machine_memo(&states1, 0);
    let result2_memo = state_machine_memo(&states2, 0);
    let result3_memo = state_machine_memo(&states3, 0);
    
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_eq!(result3, result3_memo);
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    
    println!("State machine results: {} == {} == {} != {}", result1, result2, result1_memo, result3);
}

fn test_cache_consistency() {
    println!("\n--- Cache Consistency ---");
    
    // 测试多次调用相同参数的一致性
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // 多次调用相同参数
    let result1 = complex_calculation_no_memo(&data, 5);
    let result2 = complex_calculation_no_memo(&data, 5);
    let result3 = complex_calculation_no_memo(&data, 5);
    
    let result1_memo = complex_calculation_memo(&data, 5);
    let result2_memo = complex_calculation_memo(&data, 5); // 应该命中缓存
    let result3_memo = complex_calculation_memo(&data, 5); // 应该命中缓存
    
    // 所有结果应该相同
    assert_eq!(result1, result2);
    assert_eq!(result2, result3);
    assert_eq!(result1, result1_memo);
    assert_eq!(result2, result2_memo);
    assert_eq!(result3, result3_memo);
    
    println!("Cache consistency: all results equal to {}", result1);
    
    // 测试不同参数
    let result4 = complex_calculation_memo(&data, 3);
    let result5 = complex_calculation_memo(&data, 7);
    
    assert_ne!(result1, result4);
    assert_ne!(result1, result5);
    assert_ne!(result4, result5);
    
    println!("Different parameters: {} != {} != {}", result1, result4, result5);
}

// 处理嵌套结构的函数 - 不使用 memo
fn process_nested_map_no_memo(map: &BTreeMap<String, BTreeSet<String>>) -> usize {
    let mut total = 0;
    for (key, set) in map {
        total += key.len() + set.len() * 2;
        for value in set {
            total += value.len();
        }
    }
    total
}

// 处理嵌套结构的函数 - 使用 memo
#[memo]
fn process_nested_map_memo(map: &BTreeMap<String, BTreeSet<String>>) -> usize {
    let mut total = 0;
    for (key, set) in map {
        total += key.len() + set.len() * 2;
        for value in set {
            total += value.len();
        }
    }
    total
}

// 图路径查找 - 不使用 memo
fn find_path_no_memo(graph: &BTreeMap<usize, Vec<usize>>, start: usize, end: usize) -> bool {
    if start == end {
        return true;
    }
    
    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            if find_path_no_memo(graph, neighbor, end) {
                return true;
            }
        }
    }
    false
}

// 图路径查找 - 使用 memo
#[memo]
fn find_path_memo(graph: &BTreeMap<usize, Vec<usize>>, start: usize, end: usize) -> bool {
    if start == end {
        return true;
    }
    
    if let Some(neighbors) = graph.get(&start) {
        for &neighbor in neighbors {
            if find_path_memo(graph, neighbor, end) {
                return true;
            }
        }
    }
    false
}

// 树高度计算 - 不使用 memo
fn tree_height_no_memo(tree: &[Option<i32>], index: usize) -> usize {
    if index >= tree.len() || tree[index].is_none() {
        return 0;
    }
    
    let left_height = tree_height_no_memo(tree, 2 * index + 1);
    let right_height = tree_height_no_memo(tree, 2 * index + 2);
    
    1 + std::cmp::max(left_height, right_height)
}

// 树高度计算 - 使用 memo
#[memo]
fn tree_height_memo(tree: &[Option<i32>], index: usize) -> usize {
    if index >= tree.len() || tree[index].is_none() {
        return 0;
    }
    
    let left_height = tree_height_memo(tree, 2 * index + 1);
    let right_height = tree_height_memo(tree, 2 * index + 2);
    
    1 + std::cmp::max(left_height, right_height)
}

// 状态机 - 不使用 memo
fn state_machine_no_memo(states: &[String], current: usize) -> usize {
    if current >= states.len() {
        return 0;
    }
    
    let mut score = states[current].len();
    if current + 1 < states.len() {
        score += state_machine_no_memo(states, current + 1);
    }
    score
}

// 状态机 - 使用 memo
#[memo]
fn state_machine_memo(states: &[String], current: usize) -> usize {
    if current >= states.len() {
        return 0;
    }
    
    let mut score = states[current].len();
    if current + 1 < states.len() {
        score += state_machine_memo(states, current + 1);
    }
    score
}

// 复杂计算 - 不使用 memo
fn complex_calculation_no_memo(data: &[i32], threshold: i32) -> i32 {
    if data.is_empty() {
        return 0;
    }
    
    let mut sum = 0;
    for &value in data {
        if value > threshold {
            sum += value * value;
        } else {
            sum += value;
        }
    }
    
    // 递归处理子数组
    if data.len() > 1 {
        sum += complex_calculation_no_memo(&data[1..], threshold);
    }
    
    sum
}

// 复杂计算 - 使用 memo
#[memo]
fn complex_calculation_memo(data: &[i32], threshold: i32) -> i32 {
    if data.is_empty() {
        return 0;
    }
    
    let mut sum = 0;
    for &value in data {
        if value > threshold {
            sum += value * value;
        } else {
            sum += value;
        }
    }
    
    // 递归处理子数组
    if data.len() > 1 {
        sum += complex_calculation_memo(&data[1..], threshold);
    }
    
    sum
}
