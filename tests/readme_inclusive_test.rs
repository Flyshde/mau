use mau::{min, max, sum};

#[test]
fn test_readme_inclusive_range_example() {
    let data = vec![10, 5, 8, 3, 7, 2, 9];
    
    // 排他范围 2..5 - 包含索引 2, 3, 4
    let exclusive_min = min!(|i| data[i], 2..5);
    let exclusive_max = max!(|i| data[i], 2..5);
    let exclusive_sum = sum!(|i| data[i], 2..5);
    
    // 包含范围 2..=4 - 包含索引 2, 3, 4
    let inclusive_min = min!(|i| data[i], 2..=4);
    let inclusive_max = max!(|i| data[i], 2..=4);
    let inclusive_sum = sum!(|i| data[i], 2..=4);
    
    // 验证结果
    assert_eq!(exclusive_min, 3);
    assert_eq!(inclusive_min, 3);
    assert_eq!(exclusive_max, 8);
    assert_eq!(inclusive_max, 8);
    assert_eq!(exclusive_sum, 18);
    assert_eq!(inclusive_sum, 18);
    
    // 验证范围内容
    assert_eq!(&data[2..5], &[8, 3, 7]);
    assert_eq!(&data[2..=4], &[8, 3, 7]);
    
    println!("README 包含范围示例测试通过！");
}
