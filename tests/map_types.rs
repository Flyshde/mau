use mau::memo;
use std::collections::BTreeMap;

#[test]
fn test_map_types() {
    println!("=== Testing Map Types ===");
    
    // 测试 BTreeMap<i32, i32> 类型
    let mut map1 = BTreeMap::new();
    map1.insert(1, 10);
    map1.insert(2, 20);
    map1.insert(3, 30);
    
    let mut map2 = BTreeMap::new();
    map2.insert(1, 10);
    map2.insert(2, 20);
    map2.insert(3, 30); // 相同内容，应该命中缓存
    
    let mut map3 = BTreeMap::new();
    map3.insert(1, 10);
    map3.insert(2, 20);
    map3.insert(3, 31); // 不同内容，不应该命中缓存
    
    let result1 = map_btreemap_i32(map1);
    let result2 = map_btreemap_i32(map2); // 应该命中缓存
    let result3 = map_btreemap_i32(map3); // 不应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    println!("BTreeMap<i32, i32> test: {} == {} != {}", result1, result2, result3);
    
    // 测试 BTreeMap<String, i32> 类型
    let mut map1 = BTreeMap::new();
    map1.insert("a".to_string(), 1);
    map1.insert("b".to_string(), 2);
    
    let mut map2 = BTreeMap::new();
    map2.insert("a".to_string(), 1);
    map2.insert("b".to_string(), 2); // 相同内容，应该命中缓存
    
    let result1 = map_btreemap_string(map1);
    let result2 = map_btreemap_string(map2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeMap<String, i32> test: {} == {}", result1, result2);
    
    // 测试 BTreeMap<String, Vec<i32>> 类型
    let mut map1 = BTreeMap::new();
    map1.insert("key1".to_string(), vec![1, 2, 3]);
    map1.insert("key2".to_string(), vec![4, 5, 6]);
    
    let mut map2 = BTreeMap::new();
    map2.insert("key1".to_string(), vec![1, 2, 3]);
    map2.insert("key2".to_string(), vec![4, 5, 6]); // 相同内容，应该命中缓存
    
    let result1 = map_btreemap_vec(map1);
    let result2 = map_btreemap_vec(map2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeMap<String, Vec<i32>> test: {} == {}", result1, result2);
    
    // 测试多个 map 参数
    let mut map1 = BTreeMap::new();
    map1.insert(1, 10);
    let mut map2 = BTreeMap::new();
    map2.insert(2, 20);
    
    let mut map3 = BTreeMap::new();
    map3.insert(1, 10);
    let mut map4 = BTreeMap::new();
    map4.insert(2, 20);
    
    let result1 = map_multi(map1, map2);
    let result2 = map_multi(map3, map4); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Multi BTreeMap test: {} == {}", result1, result2);
}

#[memo]
fn map_btreemap_i32(map: BTreeMap<i32, i32>) -> i32 {
    println!("Computing map_btreemap_i32({:?})", map);
    map.values().sum()
}

#[memo]
fn map_btreemap_string(map: BTreeMap<String, i32>) -> i32 {
    println!("Computing map_btreemap_string({:?})", map);
    map.values().sum()
}

#[memo]
fn map_btreemap_vec(map: BTreeMap<String, Vec<i32>>) -> i32 {
    println!("Computing map_btreemap_vec({:?})", map);
    map.values().flat_map(|v| v.iter()).sum()
}

#[memo]
fn map_multi(map1: BTreeMap<i32, i32>, map2: BTreeMap<i32, i32>) -> i32 {
    println!("Computing map_multi({:?}, {:?})", map1, map2);
    map1.values().sum::<i32>() + map2.values().sum::<i32>()
}
