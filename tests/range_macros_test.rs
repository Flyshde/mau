use mau::{min, max, sum, and, or};

#[test]
fn test_min_macro() {
    let d = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let min_val = min!(|i| d[i], 0..d.len());
    assert_eq!(min_val, 1);
    println!("Min test passed: {}", min_val);
}

#[test]
fn test_max_macro() {
    let d = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let max_val = max!(|i| d[i], 0..d.len());
    assert_eq!(max_val, 9);
    println!("Max test passed: {}", max_val);
}

#[test]
fn test_sum_macro() {
    let d = vec![1, 2, 3, 4, 5];
    let sum_val = sum!(|i| d[i], 0..d.len());
    assert_eq!(sum_val, 15);
    println!("Sum test passed: {}", sum_val);
}

#[test]
fn test_and_macro() {
    let d = vec![true, true, false, true];
    let and_val = and!(|i| d[i], 0..d.len());
    assert_eq!(and_val, false);
    println!("And test passed: {}", and_val);
    
    let all_true = vec![true, true, true, true];
    let and_val_true = and!(|i| all_true[i], 0..all_true.len());
    assert_eq!(and_val_true, true);
    println!("And (all true) test passed: {}", and_val_true);
}

#[test]
fn test_or_macro() {
    let d = vec![false, false, true, false];
    let or_val = or!(|i| d[i], 0..d.len());
    assert_eq!(or_val, true);
    println!("Or test passed: {}", or_val);
    
    let all_false = vec![false, false, false, false];
    let or_val_false = or!(|i| all_false[i], 0..all_false.len());
    assert_eq!(or_val_false, false);
    println!("Or (all false) test passed: {}", or_val_false);
}

#[test]
fn test_partial_ranges() {
    let d = vec![10, 5, 8, 3, 7, 2, 9];
    
    // 测试部分范围
    let min_partial = min!(|i| d[i], 2..5);
    assert_eq!(min_partial, 3);
    
    let max_partial = max!(|i| d[i], 2..5);
    assert_eq!(max_partial, 8);
    
    let sum_partial = sum!(|i| d[i], 2..5);
    assert_eq!(sum_partial, 18); // 8 + 3 + 7
    
    println!("Partial range tests passed");
}

#[test]
fn test_floats() {
    let floats = vec![3.5, 1.2, 4.8, 1.1, 5.9, 2.3];
    
    let min_float = min!(|i| floats[i], 0..floats.len());
    assert_eq!(min_float, 1.1);
    
    let max_float = max!(|i| floats[i], 0..floats.len());
    assert_eq!(max_float, 5.9);
    
    let sum_float = sum!(|i| floats[i], 0..floats.len());
    assert_eq!(sum_float, 18.8);
    
    println!("Float tests passed");
}

#[test]
fn test_complex_expressions() {
    let data = vec![1, 2, 3, 4, 5];
    
    // 测试复杂表达式
    let min_squared = min!(|i| data[i] * data[i], 0..data.len());
    assert_eq!(min_squared, 1); // 1*1 = 1
    
    let max_doubled = max!(|i| data[i] * 2, 0..data.len());
    assert_eq!(max_doubled, 10); // 5*2 = 10
    
    let sum_plus_one = sum!(|i| data[i] + 1, 0..data.len());
    assert_eq!(sum_plus_one, 20); // (1+1) + (2+1) + (3+1) + (4+1) + (5+1) = 20
    
    println!("Complex expression tests passed");
}
