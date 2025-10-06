use mau::{min, max, sum, and, or};

#[test]
fn test_min_multi_args() {
    // 测试多参数min宏
    let a = 5;
    let b = 3;
    let c = 8;
    
    let result = min!(1, a, b, c, 3);
    assert_eq!(result, 1);
    
    let result2 = min!(a, b, c);
    assert_eq!(result2, 3);
    
    let result3 = min!(42);
    assert_eq!(result3, 42);
}

#[test]
fn test_max_multi_args() {
    // 测试多参数max宏
    let a = 5;
    let b = 3;
    let c = 8;
    
    let result = max!(1, a, b, c, 3);
    assert_eq!(result, 8);
    
    let result2 = max!(a, b, c);
    assert_eq!(result2, 8);
    
    let result3 = max!(42);
    assert_eq!(result3, 42);
}

#[test]
fn test_sum_multi_args() {
    // 测试多参数sum宏
    let a = 5;
    let b = 3;
    let c = 8;
    
    let result = sum!(1, a, b, c, 3);
    assert_eq!(result, 20); // 1 + 5 + 3 + 8 + 3 = 20
    
    let result2 = sum!(a, b, c);
    assert_eq!(result2, 16); // 5 + 3 + 8 = 16
    
    let result3 = sum!(42);
    assert_eq!(result3, 42);
}

#[test]
fn test_and_multi_args() {
    // 测试多参数and宏
    let a = true;
    let b = true;
    let c = false;
    
    let result = and!(true, a, b, c, true);
    assert_eq!(result, false);
    
    let result2 = and!(a, b, c);
    assert_eq!(result2, false);
    
    let result3 = and!(true, true, true);
    assert_eq!(result3, true);
    
    let result4 = and!(true);
    assert_eq!(result4, true);
}

#[test]
fn test_or_multi_args() {
    // 测试多参数or宏
    let a = false;
    let b = false;
    let c = true;
    
    let result = or!(false, a, b, c, false);
    assert_eq!(result, true);
    
    let result2 = or!(a, b, c);
    assert_eq!(result2, true);
    
    let result3 = or!(false, false, false);
    assert_eq!(result3, false);
    
    let result4 = or!(true);
    assert_eq!(result4, true);
}

#[test]
fn test_mixed_types() {
    // 测试混合类型
    let x = 10;
    let y = 20;
    let z = 15;
    
    // 整数
    let min_int = min!(x, y, z);
    assert_eq!(min_int, 10);
    
    let max_int = max!(x, y, z);
    assert_eq!(max_int, 20);
    
    let sum_int = sum!(x, y, z);
    assert_eq!(sum_int, 45);
    
    // 浮点数
    let a = 1.5;
    let b = 2.5;
    let c = 3.5;
    
    let min_float = min!(a, b, c);
    assert_eq!(min_float, 1.5);
    
    let max_float = max!(a, b, c);
    assert_eq!(max_float, 3.5);
    
    let sum_float = sum!(a, b, c);
    assert_eq!(sum_float, 7.5);
}

#[test]
fn test_expressions() {
    // 测试表达式
    let x = 5;
    let y = 10;
    
    let result = min!(x * 2, y + 1, 15);
    assert_eq!(result, 10); // min(10, 11, 15) = 10
    
    let result2 = max!(x * 2, y + 1, 15);
    assert_eq!(result2, 15); // max(10, 11, 15) = 15
    
    let result3 = sum!(x * 2, y + 1, 15);
    assert_eq!(result3, 36); // 10 + 11 + 15 = 36
}
