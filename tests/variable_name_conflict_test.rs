use mau::{min, max, sum, and, or};

#[test]
fn test_variable_name_conflict() {
    // 测试宏展开中的变量名不会与用户代码中的变量名冲突
    let i = 5;
    let j = 3;
    let k = 8;
    
    // 多参数语法测试
    let min_val = min!(i, j, k);
    assert_eq!(min_val, 3);
    
    let max_val = max!(i, j, k);
    assert_eq!(max_val, 8);
    
    let sum_val = sum!(i, j, k);
    assert_eq!(sum_val, 16);
    
    let and_val = and!(i > 0, j > 0, k > 0);
    assert_eq!(and_val, true);
    
    let or_val = or!(i < 0, j < 0, k > 0);
    assert_eq!(or_val, true);
    
    // 范围语法测试
    let data = vec![i, j, k];
    let range_min = min!(|idx| data[idx], [0..data.len()]);
    assert_eq!(range_min, 3);
    
    let range_max = max!(|idx| data[idx], [0..data.len()]);
    assert_eq!(range_max, 8);
    
    let range_sum = sum!(|idx| data[idx], [0..data.len()]);
    assert_eq!(range_sum, 16);
    
    let bools = vec![i > 0, j > 0, k > 0];
    let range_and = and!(|idx| bools[idx], [0..bools.len()]);
    assert_eq!(range_and, true);
    
    let range_or = or!(|idx| bools[idx], [0..bools.len()]);
    assert_eq!(range_or, true);
    
    // 验证用户变量没有被修改
    assert_eq!(i, 5);
    assert_eq!(j, 3);
    assert_eq!(k, 8);
}

#[test]
fn test_common_variable_names() {
    // 测试常见的变量名不会与宏内部变量冲突
    let i = 1;
    let j = 2;
    let k = 3;
    let idx = 4;
    let index = 5;
    let val = 6;
    let value = 7;
    let current = 8;
    let acc = 9;
    let sum = 10;
    let min = 11;
    let max = 12;
    
    // 使用这些变量名进行多参数操作
    let result = min!(i, j, k, idx, index, val, value, current, acc, sum, min, max);
    assert_eq!(result, 1);
    
    let result2 = max!(i, j, k, idx, index, val, value, current, acc, sum, min, max);
    assert_eq!(result2, 12);
    
    let result3 = sum!(i, j, k, idx, index, val, value, current, acc, sum, min, max);
    assert_eq!(result3, 78); // 1+2+3+4+5+6+7+8+9+10+11+12 = 78
    
    // 验证所有变量都没有被修改
    assert_eq!(i, 1);
    assert_eq!(j, 2);
    assert_eq!(k, 3);
    assert_eq!(idx, 4);
    assert_eq!(index, 5);
    assert_eq!(val, 6);
    assert_eq!(value, 7);
    assert_eq!(current, 8);
    assert_eq!(acc, 9);
    assert_eq!(sum, 10);
    assert_eq!(min, 11);
    assert_eq!(max, 12);
}
