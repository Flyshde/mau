use mau::{min, max, sum, and, or};

#[test]
fn test_min_two_args() {
    // 测试两个参数的情况
    let result = min!(1, 2);
    assert_eq!(result, 1);
    
    let result = min!(5, 3);
    assert_eq!(result, 3);
    
    // 测试变量
    let a = 10;
    let b = 20;
    let result = min!(a, b);
    assert_eq!(result, 10);
    
    // 测试浮点数
    let result = min!(3.14, 2.71);
    assert_eq!(result, 2.71);
}

#[test]
fn test_max_two_args() {
    let result = max!(1, 2);
    assert_eq!(result, 2);
    
    let result = max!(5, 3);
    assert_eq!(result, 5);
    
    let a = 10;
    let b = 20;
    let result = max!(a, b);
    assert_eq!(result, 20);
}

#[test]
fn test_sum_two_args() {
    let result = sum!(1, 2);
    assert_eq!(result, 3);
    
    let result = sum!(5, 7);
    assert_eq!(result, 12);
    
    let a = 10;
    let b = 20;
    let result = sum!(a, b);
    assert_eq!(result, 30);
}

#[test]
fn test_and_two_args() {
    let result = and!(true, true);
    assert_eq!(result, true);
    
    let result = and!(true, false);
    assert_eq!(result, false);
    
    let result = and!(false, true);
    assert_eq!(result, false);
    
    let result = and!(false, false);
    assert_eq!(result, false);
}

#[test]
fn test_or_two_args() {
    let result = or!(true, true);
    assert_eq!(result, true);
    
    let result = or!(true, false);
    assert_eq!(result, true);
    
    let result = or!(false, true);
    assert_eq!(result, true);
    
    let result = or!(false, false);
    assert_eq!(result, false);
}

#[test]
fn test_two_args_with_expressions() {
    // 测试表达式作为参数
    let x = 5;
    let y = 10;
    
    let result = min!(x * 2, y + 5);
    assert_eq!(result, 10); // min(10, 15) = 10
    
    let result = max!(x * 2, y + 5);
    assert_eq!(result, 15); // max(10, 15) = 15
    
    let result = sum!(x * 2, y + 5);
    assert_eq!(result, 25); // 10 + 15 = 25
}

#[test]
fn test_two_args_negative_numbers() {
    let result = min!(-5, 3);
    assert_eq!(result, -5);
    
    let result = max!(-5, 3);
    assert_eq!(result, 3);
    
    let result = min!(-10, -20);
    assert_eq!(result, -20);
    
    let result = max!(-10, -20);
    assert_eq!(result, -10);
    
    let result = sum!(-5, 3);
    assert_eq!(result, -2);
}

#[test]
fn test_two_args_same_values() {
    let result = min!(5, 5);
    assert_eq!(result, 5);
    
    let result = max!(5, 5);
    assert_eq!(result, 5);
    
    let result = sum!(5, 5);
    assert_eq!(result, 10);
}

#[test]
fn test_two_args_characters() {
    let result = min!('a', 'z');
    assert_eq!(result, 'a');
    
    let result = max!('a', 'z');
    assert_eq!(result, 'z');
}

#[test]
fn test_multi_args_still_works() {
    // 确保修复后多参数（3个或更多）仍然正常工作
    let result = min!(1, 2, 3);
    assert_eq!(result, 1);
    
    let result = max!(1, 2, 3);
    assert_eq!(result, 3);
    
    let result = sum!(1, 2, 3);
    assert_eq!(result, 6);
    
    // 测试更多参数
    let result = min!(5, 2, 8, 1, 9, 3);
    assert_eq!(result, 1);
    
    let result = max!(5, 2, 8, 1, 9, 3);
    assert_eq!(result, 9);
    
    let result = sum!(1, 2, 3, 4, 5);
    assert_eq!(result, 15);
}

#[test]
fn test_range_syntax_still_works() {
    // 确保修复后范围语法仍然正常工作
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    let result = min!(|i| data[i], 0..data.len());
    assert_eq!(result, 1);
    
    let result = max!(|i| data[i], 0..data.len());
    assert_eq!(result, 9);
    
    let result = sum!(|i| data[i], 0..data.len());
    assert_eq!(result, 31);
}

