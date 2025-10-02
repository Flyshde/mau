use mau::memo;
use std::collections::BTreeSet;

#[test]
fn test_set_types() {
    println!("=== Testing Set Types ===");
    
    // 测试 BTreeSet<i32> 类型
    let mut set1 = BTreeSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    set1.insert(5);
    
    let mut set2 = BTreeSet::new();
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);
    set2.insert(5); // 相同内容，应该命中缓存
    
    let mut set3 = BTreeSet::new();
    set3.insert(1);
    set3.insert(2);
    set3.insert(3);
    set3.insert(4);
    set3.insert(6); // 不同内容，不应该命中缓存
    
    let result1 = set_btreeset_i32(set1);
    let result2 = set_btreeset_i32(set2); // 应该命中缓存
    let result3 = set_btreeset_i32(set3); // 不应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    println!("BTreeSet<i32> test: {} == {} != {}", result1, result2, result3);
    
    // 测试 BTreeSet<String> 类型
    let mut set1 = BTreeSet::new();
    set1.insert("hello".to_string());
    set1.insert("world".to_string());
    set1.insert("rust".to_string());
    
    let mut set2 = BTreeSet::new();
    set2.insert("hello".to_string());
    set2.insert("world".to_string());
    set2.insert("rust".to_string()); // 相同内容，应该命中缓存
    
    let result1 = set_btreeset_string(set1);
    let result2 = set_btreeset_string(set2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeSet<String> test: {} == {}", result1, result2);
    
    // 测试 BTreeSet<char> 类型
    let mut set1 = BTreeSet::new();
    set1.insert('a');
    set1.insert('b');
    set1.insert('c');
    set1.insert('d');
    
    let mut set2 = BTreeSet::new();
    set2.insert('a');
    set2.insert('b');
    set2.insert('c');
    set2.insert('d'); // 相同内容，应该命中缓存
    
    let result1 = set_btreeset_char(set1);
    let result2 = set_btreeset_char(set2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeSet<char> test: {} == {}", result1, result2);
    
    // 测试 BTreeSet<Vec<i32>> 类型
    let mut set1 = BTreeSet::new();
    set1.insert(vec![1, 2, 3]);
    set1.insert(vec![4, 5, 6]);
    set1.insert(vec![7, 8, 9]);
    
    let mut set2 = BTreeSet::new();
    set2.insert(vec![1, 2, 3]);
    set2.insert(vec![4, 5, 6]);
    set2.insert(vec![7, 8, 9]); // 相同内容，应该命中缓存
    
    let result1 = set_btreeset_vec(set1);
    let result2 = set_btreeset_vec(set2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeSet<Vec<i32>> test: {} == {}", result1, result2);
    
    // 测试多个 set 参数
    let mut set1 = BTreeSet::new();
    set1.insert(1);
    set1.insert(2);
    let mut set2 = BTreeSet::new();
    set2.insert(3);
    set2.insert(4);
    
    let mut set3 = BTreeSet::new();
    set3.insert(1);
    set3.insert(2);
    let mut set4 = BTreeSet::new();
    set4.insert(3);
    set4.insert(4);
    
    let result1 = set_multi(set1, set2);
    let result2 = set_multi(set3, set4); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Multi BTreeSet test: {} == {}", result1, result2);
    
    // 测试 BTreeSet 与 Vec 的混合参数
    let mut set1 = BTreeSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    
    let vec1 = vec![4, 5, 6];
    let vec2 = vec![4, 5, 6]; // 相同内容，应该命中缓存
    
    let result1 = set_vec_mixed(set1.clone(), vec1);
    let result2 = set_vec_mixed(set1, vec2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeSet + Vec mixed test: {} == {}", result1, result2);
    
    // 测试空集合
    let empty_set1 = BTreeSet::new();
    let empty_set2 = BTreeSet::new();
    
    let result1 = set_btreeset_i32(empty_set1);
    let result2 = set_btreeset_i32(empty_set2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Empty BTreeSet test: {} == {}", result1, result2);
}

#[memo]
fn set_btreeset_i32(set: BTreeSet<i32>) -> i32 {
    println!("Computing set_btreeset_i32({:?})", set);
    set.iter().sum()
}

#[memo]
fn set_btreeset_string(set: BTreeSet<String>) -> usize {
    println!("Computing set_btreeset_string({:?})", set);
    set.iter().map(|s| s.len()).sum()
}

#[memo]
fn set_btreeset_char(set: BTreeSet<char>) -> usize {
    println!("Computing set_btreeset_char({:?})", set);
    set.len()
}

#[memo]
fn set_btreeset_vec(set: BTreeSet<Vec<i32>>) -> i32 {
    println!("Computing set_btreeset_vec({:?})", set);
    set.iter().flat_map(|v| v.iter()).sum()
}

#[memo]
fn set_multi(set1: BTreeSet<i32>, set2: BTreeSet<i32>) -> i32 {
    println!("Computing set_multi({:?}, {:?})", set1, set2);
    set1.iter().sum::<i32>() + set2.iter().sum::<i32>()
}

#[memo]
fn set_vec_mixed(set: BTreeSet<i32>, vec: Vec<i32>) -> i32 {
    println!("Computing set_vec_mixed({:?}, {:?})", set, vec);
    set.iter().sum::<i32>() + vec.iter().sum::<i32>()
}
