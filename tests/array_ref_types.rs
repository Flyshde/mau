use mau::memo;

#[test]
fn test_array_ref_types() {
    println!("=== Testing Array Reference Types ===");
    
    // 测试 &[i32] 类型
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5]; // 相同内容，应该命中缓存
    let arr3 = [1, 2, 3, 4, 6]; // 不同内容，不应该命中缓存
    
    let result1 = array_ref_i32(&arr1);
    let result2 = array_ref_i32(&arr2); // 应该命中缓存
    let result3 = array_ref_i32(&arr3); // 不应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    println!("&[i32] test: {} == {} != {}", result1, result2, result3);
    
    // 测试 &[f64] 类型
    let arr1 = [1.0, 2.0, 3.0, 4.0, 5.0];
    let arr2 = [1.0, 2.0, 3.0, 4.0, 5.0]; // 相同内容，应该命中缓存
    
    let result1 = array_ref_f64(&arr1);
    let result2 = array_ref_f64(&arr2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("&[f64] test: {} == {}", result1, result2);
    
    // 测试 &[bool] 类型
    let arr1 = [true, false, true, false];
    let arr2 = [true, false, true, false]; // 相同内容，应该命中缓存
    
    let result1 = array_ref_bool(&arr1);
    let result2 = array_ref_bool(&arr2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("&[bool] test: {} == {}", result1, result2);
    
    // 测试 &[char] 类型
    let arr1 = ['a', 'b', 'c', 'd'];
    let arr2 = ['a', 'b', 'c', 'd']; // 相同内容，应该命中缓存
    
    let result1 = array_ref_char(&arr1);
    let result2 = array_ref_char(&arr2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("&[char] test: {} == {}", result1, result2);
    
    // 测试多个数组引用参数
    let arr1 = [1, 2, 3];
    let arr2 = [4, 5, 6];
    let arr3 = [1, 2, 3];
    let arr4 = [4, 5, 6];
    
    let result1 = array_ref_multi(&arr1, &arr2);
    let result2 = array_ref_multi(&arr3, &arr4); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Multi &[i32] test: {} == {}", result1, result2);
    
    // 测试混合类型参数
    let arr1 = [1, 2, 3];
    let arr2 = [1.0, 2.0, 3.0];
    let arr3 = [1, 2, 3];
    let arr4 = [1.0, 2.0, 3.0];
    
    let result1 = array_ref_mixed(&arr1, &arr2);
    let result2 = array_ref_mixed(&arr3, &arr4); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Mixed types test: {} == {}", result1, result2);
}

#[memo]
fn array_ref_i32(arr: &[i32]) -> i32 {
    println!("Computing array_ref_i32({:?})", arr);
    arr.iter().sum()
}

#[memo]
fn array_ref_f64(arr: &[f64]) -> f64 {
    println!("Computing array_ref_f64({:?})", arr);
    arr.iter().sum()
}

#[memo]
fn array_ref_bool(arr: &[bool]) -> usize {
    println!("Computing array_ref_bool({:?})", arr);
    arr.iter().filter(|&&b| b).count()
}

#[memo]
fn array_ref_char(arr: &[char]) -> usize {
    println!("Computing array_ref_char({:?})", arr);
    arr.len()
}

#[memo]
fn array_ref_multi(arr1: &[i32], arr2: &[i32]) -> i32 {
    println!("Computing array_ref_multi({:?}, {:?})", arr1, arr2);
    arr1.iter().sum::<i32>() + arr2.iter().sum::<i32>()
}

#[memo]
fn array_ref_mixed(arr1: &[i32], arr2: &[f64]) -> f64 {
    println!("Computing array_ref_mixed({:?}, {:?})", arr1, arr2);
    arr1.iter().sum::<i32>() as f64 + arr2.iter().sum::<f64>()
}
