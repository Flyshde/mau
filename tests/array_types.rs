use mau::memo;

#[test]
fn test_array_types() {
    println!("=== Testing Array Types ===");
    
    // 测试 Vec<i32> 类型
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![1, 2, 3, 4, 5]; // 相同内容，应该命中缓存
    let vec3 = vec![1, 2, 3, 4, 6]; // 不同内容，不应该命中缓存
    
    let result1 = array_vec_i32(vec1);
    let result2 = array_vec_i32(vec2); // 应该命中缓存
    let result3 = array_vec_i32(vec3); // 不应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    println!("Vec<i32> test: {} == {} != {}", result1, result2, result3);
    
    // 测试 Vec<String> 类型
    let vec1 = vec!["hello".to_string(), "world".to_string()];
    let vec2 = vec!["hello".to_string(), "world".to_string()]; // 相同内容，应该命中缓存
    
    let result1 = array_vec_string(vec1);
    let result2 = array_vec_string(vec2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Vec<String> test: {} == {}", result1, result2);
    
    // 测试 Vec<bool> 类型
    let vec1 = vec![true, false, true, false];
    let vec2 = vec![true, false, true, false]; // 相同内容，应该命中缓存
    
    let result1 = array_vec_bool(vec1);
    let result2 = array_vec_bool(vec2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Vec<bool> test: {} == {}", result1, result2);
    
    // 测试多个 Vec 参数
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = vec![1, 2, 3];
    let vec4 = vec![4, 5, 6];
    
    let result1 = array_multi_vec(vec1, vec2);
    let result2 = array_multi_vec(vec3, vec4); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Multi Vec test: {} == {}", result1, result2);
}

#[memo]
fn array_vec_i32(vec: Vec<i32>) -> i32 {
    println!("Computing array_vec_i32({:?})", vec);
    vec.iter().sum()
}

#[memo]
fn array_vec_string(vec: Vec<String>) -> usize {
    println!("Computing array_vec_string({:?})", vec);
    vec.iter().map(|s| s.len()).sum()
}

#[memo]
fn array_vec_bool(vec: Vec<bool>) -> usize {
    println!("Computing array_vec_bool({:?})", vec);
    vec.iter().filter(|&&b| b).count()
}

#[memo]
fn array_multi_vec(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    println!("Computing array_multi_vec({:?}, {:?})", vec1, vec2);
    vec1.iter().sum::<i32>() + vec2.iter().sum::<i32>()
}
