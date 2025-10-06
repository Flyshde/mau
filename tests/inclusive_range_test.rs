use mau::{min, max, sum, and, or};

#[test]
fn test_inclusive_ranges() {
    println!("=== 测试包含范围 (..=) ===");
    
    let data = vec![10, 20, 30, 40, 50];
    
    // 测试包含范围 1..=3 应该包含索引 1, 2, 3
    let min_val = min!(|i| data[i], 1..=3);
    let max_val = max!(|i| data[i], 1..=3);
    let sum_val = sum!(|i| data[i], 1..=3);
    
    println!("数据: {:?}", data);
    println!("范围 1..=3: {:?}", &data[1..=3]);
    println!("包含范围最小值: {}", min_val);
    println!("包含范围最大值: {}", max_val);
    println!("包含范围总和: {}", sum_val);
    
    // 验证结果
    assert_eq!(min_val, 20); // data[1] = 20
    assert_eq!(max_val, 40); // data[3] = 40
    assert_eq!(sum_val, 90); // 20 + 30 + 40 = 90
}

#[test]
fn test_exclusive_vs_inclusive() {
    println!("=== 比较排他范围和包含范围 ===");
    
    let data = vec![100, 200, 300, 400, 500];
    
    // 排他范围 1..4 包含索引 1, 2, 3
    let exclusive_min = min!(|i| data[i], 1..4);
    let exclusive_sum = sum!(|i| data[i], 1..4);
    
    // 包含范围 1..=3 包含索引 1, 2, 3
    let inclusive_min = min!(|i| data[i], 1..=3);
    let inclusive_sum = sum!(|i| data[i], 1..=3);
    
    println!("数据: {:?}", data);
    println!("排他范围 1..4: {:?}", &data[1..4]);
    println!("包含范围 1..=3: {:?}", &data[1..=3]);
    println!("排他范围最小值: {}", exclusive_min);
    println!("包含范围最小值: {}", inclusive_min);
    println!("排他范围总和: {}", exclusive_sum);
    println!("包含范围总和: {}", inclusive_sum);
    
    // 两种范围应该产生相同的结果
    assert_eq!(exclusive_min, inclusive_min);
    assert_eq!(exclusive_sum, inclusive_sum);
}

#[test]
fn test_boolean_inclusive_ranges() {
    println!("=== 测试布尔值的包含范围 ===");
    
    let bools = vec![true, false, true, false, true];
    
    // 包含范围 1..=3 包含索引 1, 2, 3
    let and_result = and!(|i| bools[i], 1..=3);
    let or_result = or!(|i| bools[i], 1..=3);
    
    println!("布尔数组: {:?}", bools);
    println!("包含范围 1..=3: {:?}", &bools[1..=3]);
    println!("包含范围逻辑与: {}", and_result);
    println!("包含范围逻辑或: {}", or_result);
    
    // 验证结果
    assert_eq!(and_result, false); // false && true && false = false
    assert_eq!(or_result, true);   // false || true || false = true
}

#[test]
fn test_single_element_inclusive_range() {
    println!("=== 测试单元素包含范围 ===");
    
    let data = vec![42];
    
    // 0..=0 应该包含索引 0
    let min_val = min!(|i| data[i], 0..=0);
    let max_val = max!(|i| data[i], 0..=0);
    let sum_val = sum!(|i| data[i], 0..=0);
    
    println!("数据: {:?}", data);
    println!("单元素包含范围 0..=0: {:?}", &data[0..=0]);
    println!("单元素最小值: {}", min_val);
    println!("单元素最大值: {}", max_val);
    println!("单元素总和: {}", sum_val);
    
    // 验证结果
    assert_eq!(min_val, 42);
    assert_eq!(max_val, 42);
    assert_eq!(sum_val, 42);
}
