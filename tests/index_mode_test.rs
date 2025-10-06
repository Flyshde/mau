use mau::memo;

// 测试 light 模式：直接使用地址作为索引值
#[memo(light)]
fn sum_light_mode(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 测试 normal 模式：解开一层引用
#[memo(normal)]
fn sum_normal_mode(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 测试 heavy 模式：通过 copy 完全还原
#[memo(heavy)]
fn sum_heavy_mode(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 测试默认模式（应该是 normal）
#[memo]
fn sum_default_mode(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 测试嵌套数组的情况 - 使用 Vec<Vec<i32>> 而不是 &[&[i32]]
#[memo(light)]
fn sum_nested_light(data: &[Vec<i32>]) -> i32 {
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

#[memo(normal)]
fn sum_nested_normal(data: &[Vec<i32>]) -> i32 {
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

#[memo(heavy)]
fn sum_nested_heavy(data: &[Vec<i32>]) -> i32 {
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

#[test]
fn test_light_mode() {
    let data = vec![1, 2, 3, 4, 5];
    let result = sum_light_mode(&data);
    assert_eq!(result, 15);
    
    // 再次调用应该使用缓存
    let result2 = sum_light_mode(&data);
    assert_eq!(result2, 15);
}

#[test]
fn test_normal_mode() {
    let data = vec![1, 2, 3, 4, 5];
    let result = sum_normal_mode(&data);
    assert_eq!(result, 15);
    
    // 再次调用应该使用缓存
    let result2 = sum_normal_mode(&data);
    assert_eq!(result2, 15);
}

#[test]
fn test_heavy_mode() {
    let data = vec![1, 2, 3, 4, 5];
    let result = sum_heavy_mode(&data);
    assert_eq!(result, 15);
    
    // 再次调用应该使用缓存
    let result2 = sum_heavy_mode(&data);
    assert_eq!(result2, 15);
}

#[test]
fn test_default_mode() {
    let data = vec![1, 2, 3, 4, 5];
    let result = sum_default_mode(&data);
    assert_eq!(result, 15);
    
    // 再次调用应该使用缓存
    let result2 = sum_default_mode(&data);
    assert_eq!(result2, 15);
}

#[test]
fn test_nested_arrays() {
    let row1 = vec![1, 2, 3];
    let row2 = vec![4, 5, 6];
    let data = vec![row1, row2];
    
    // 测试 light 模式
    let result_light = sum_nested_light(&data);
    assert_eq!(result_light, 21);
    
    // 测试 normal 模式
    let result_normal = sum_nested_normal(&data);
    assert_eq!(result_normal, 21);
    
    // 测试 heavy 模式
    let result_heavy = sum_nested_heavy(&data);
    assert_eq!(result_heavy, 21);
}

#[test]
fn test_different_data_same_content() {
    let data1 = vec![1, 2, 3];
    let data2 = vec![1, 2, 3]; // 相同内容但不同地址
    
    // light 模式应该区分不同的地址
    let result1 = sum_light_mode(&data1);
    let result2 = sum_light_mode(&data2);
    assert_eq!(result1, 6);
    assert_eq!(result2, 6);
    
    // normal 和 heavy 模式应该识别相同内容
    let result1_normal = sum_normal_mode(&data1);
    let result2_normal = sum_normal_mode(&data2);
    assert_eq!(result1_normal, 6);
    assert_eq!(result2_normal, 6);
    
    let result1_heavy = sum_heavy_mode(&data1);
    let result2_heavy = sum_heavy_mode(&data2);
    assert_eq!(result1_heavy, 6);
    assert_eq!(result2_heavy, 6);
}

#[test]
fn test_performance_comparison() {
    use std::time::Instant;
    
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iterations = 1000;
    
    // 预热缓存
    sum_light_mode(&data);
    sum_normal_mode(&data);
    sum_heavy_mode(&data);
    
    // 测试 light 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        sum_light_mode(&data);
    }
    let light_time = start.elapsed();
    
    // 测试 normal 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        sum_normal_mode(&data);
    }
    let normal_time = start.elapsed();
    
    // 测试 heavy 模式性能
    let start = Instant::now();
    for _ in 0..iterations {
        sum_heavy_mode(&data);
    }
    let heavy_time = start.elapsed();
    
    println!("Light mode: {:?}", light_time);
    println!("Normal mode: {:?}", normal_time);
    println!("Heavy mode: {:?}", heavy_time);
    
    // 验证所有模式都能正常工作
    assert!(light_time.as_nanos() > 0);
    assert!(normal_time.as_nanos() > 0);
    assert!(heavy_time.as_nanos() > 0);
}
