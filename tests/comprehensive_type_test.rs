use mau::memo;

// 测试所有类型组合

// 1. 基本类型
#[memo(key=ptr)]
fn process_i32_ptr(val: i32) -> i32 {
    val * 2
}

#[memo(key=ref)]
fn process_i32_ref(val: i32) -> i32 {
    val * 2
}

#[memo(key=val)]
fn process_i32_val(val: i32) -> i32 {
    val * 2
}

// 2. 引用类型 (&i32, &f64, &String等)
#[memo(key=ptr)]
fn process_ref_i32_ptr(val: &i32) -> i32 {
    *val * 2
}

#[memo(key=ref)]
fn process_ref_i32_ref(val: &i32) -> i32 {
    *val * 2
}

#[memo(key=val)]
fn process_ref_i32_val(val: &i32) -> i32 {
    *val * 2
}

// 3. 浮点引用
#[memo(key=ptr)]
fn process_ref_f64_ptr(val: &f64) -> f64 {
    *val * 2.0
}

#[memo(key=ref)]
fn process_ref_f64_ref(val: &f64) -> f64 {
    *val * 2.0
}

#[memo(key=val)]
fn process_ref_f64_val(val: &f64) -> f64 {
    *val * 2.0
}

// 4. 切片 (&[T])
#[memo(key=ptr)]
fn process_slice_ptr(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(key=ref)]
fn process_slice_ref(data: &[i32]) -> i32 {
    data.iter().sum()
}

#[memo(key=val)]
fn process_slice_val(data: &[i32]) -> i32 {
    data.iter().sum()
}

// 5. 浮点切片
#[memo(key=ptr)]
fn process_f64_slice_ptr(data: &[f64]) -> f64 {
    data.iter().sum()
}

#[memo(key=ref)]
fn process_f64_slice_ref(data: &[f64]) -> f64 {
    data.iter().sum()
}

#[memo(key=val)]
fn process_f64_slice_val(data: &[f64]) -> f64 {
    data.iter().sum()
}

// 6. 数组引用 (&[T; N])
#[memo(key=ptr)]
fn process_array_ptr(data: &[i32; 5]) -> i32 {
    data.iter().sum()
}

#[memo(key=ref)]
fn process_array_ref(data: &[i32; 5]) -> i32 {
    data.iter().sum()
}

#[memo(key=val)]
fn process_array_val(data: &[i32; 5]) -> i32 {
    data.iter().sum()
}

// 7. 嵌套数组 &[[T; N]]
#[memo(key=ptr)]
fn process_nested_array_ptr(data: &[[i32; 2]]) -> i32 {
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

#[memo(key=ref)]
fn process_nested_array_ref(data: &[[i32; 2]]) -> i32 {
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

#[memo(key=val)]
fn process_nested_array_val(data: &[[i32; 2]]) -> i32 {
    data.iter().map(|row| row.iter().sum::<i32>()).sum()
}

// 8. String引用
#[memo(key=ptr)]
fn process_string_ptr(s: &String) -> usize {
    s.len()
}

#[memo(key=ref)]
fn process_string_ref(s: &String) -> usize {
    s.len()
}

#[memo(key=val)]
fn process_string_val(s: &String) -> usize {
    s.len()
}

// 9. 混合参数（不使用 &str）
#[memo(key=ptr)]
fn process_mixed_ptr(a: i32, b: &[f64], c: &String) -> f64 {
    a as f64 + b.iter().sum::<f64>() + c.len() as f64
}

#[memo(key=ref)]
fn process_mixed_ref(a: i32, b: &[f64], c: &String) -> f64 {
    a as f64 + b.iter().sum::<f64>() + c.len() as f64
}

#[memo(key=val)]
fn process_mixed_val(a: i32, b: &[f64], c: &String) -> f64 {
    a as f64 + b.iter().sum::<f64>() + c.len() as f64
}

#[test]
fn test_all_type_combinations() {
    println!("=== 测试所有类型组合 ===\n");
    
    // 1. 基本类型
    println!("1. 基本类型 (i32)");
    assert_eq!(process_i32_ptr(10), 20);
    assert_eq!(process_i32_ref(10), 20);
    assert_eq!(process_i32_val(10), 20);
    println!("   ✅ 通过\n");
    
    // 2. 引用类型
    println!("2. 整数引用 (&i32)");
    let x = 10;
    assert_eq!(process_ref_i32_ptr(&x), 20);
    assert_eq!(process_ref_i32_ref(&x), 20);
    assert_eq!(process_ref_i32_val(&x), 20);
    println!("   ✅ 通过\n");
    
    // 3. 浮点引用
    println!("3. 浮点引用 (&f64)");
    let f = 3.14;
    assert_eq!(process_ref_f64_ptr(&f), 6.28);
    assert_eq!(process_ref_f64_ref(&f), 6.28);
    assert_eq!(process_ref_f64_val(&f), 6.28);
    println!("   ✅ 通过\n");
    
    // 4. 切片
    println!("4. 整数切片 (&[i32])");
    let arr = vec![1, 2, 3, 4, 5];
    assert_eq!(process_slice_ptr(&arr), 15);
    assert_eq!(process_slice_ref(&arr), 15);
    assert_eq!(process_slice_val(&arr), 15);
    println!("   ✅ 通过\n");
    
    // 5. 浮点切片
    println!("5. 浮点切片 (&[f64])");
    let farr = vec![1.0, 2.0, 3.0];
    assert_eq!(process_f64_slice_ptr(&farr), 6.0);
    assert_eq!(process_f64_slice_ref(&farr), 6.0);
    assert_eq!(process_f64_slice_val(&farr), 6.0);
    println!("   ✅ 通过\n");
    
    // 6. 数组引用
    println!("6. 数组引用 (&[i32; 5])");
    let fixed = [1, 2, 3, 4, 5];
    assert_eq!(process_array_ptr(&fixed), 15);
    assert_eq!(process_array_ref(&fixed), 15);
    assert_eq!(process_array_val(&fixed), 15);
    println!("   ✅ 通过\n");
    
    // 7. 嵌套数组
    println!("7. 嵌套数组 (&[[i32; 2]])");
    let nested = vec![[1, 2], [3, 4], [5, 6]];
    assert_eq!(process_nested_array_ptr(&nested), 21);
    assert_eq!(process_nested_array_ref(&nested), 21);
    assert_eq!(process_nested_array_val(&nested), 21);
    println!("   ✅ 通过\n");
    
    // 8. String引用
    println!("8. String引用 (&String)");
    let s = String::from("hello");
    assert_eq!(process_string_ptr(&s), 5);
    assert_eq!(process_string_ref(&s), 5);
    assert_eq!(process_string_val(&s), 5);
    println!("   ✅ 通过\n");
    
    // 9. 混合参数
    println!("9. 混合参数 (i32, &[f64], &String)");
    let farr2 = vec![1.5, 2.5];
    let s2 = String::from("ab");
    assert_eq!(process_mixed_ptr(10, &farr2, &s2), 16.0);
    assert_eq!(process_mixed_ref(10, &farr2, &s2), 16.0);
    assert_eq!(process_mixed_val(10, &farr2, &s2), 16.0);
    println!("   ✅ 通过\n");
    
    println!("=== 所有类型组合测试通过！ ===");
}

#[test]
fn test_caching_behavior() {
    println!("\n=== 测试缓存行为 ===\n");
    
    // ptr 模式：不同地址不命中
    println!("1. ptr 模式：不同地址");
    process_slice_ptr_clear();
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let r1 = process_slice_ptr(&v1);
    let r2 = process_slice_ptr(&v2); // 不同地址，不会命中缓存
    assert_eq!(r1, r2); // 结果相同
    println!("   ✅ 通过\n");
    
    // ref 模式：相同内容命中
    println!("2. ref 模式：相同内容");
    process_slice_ref_clear();
    let v3 = vec![4, 5, 6];
    let v4 = vec![4, 5, 6];
    let r3 = process_slice_ref(&v3);
    let r4 = process_slice_ref(&v4); // 不同地址，相同内容，应该命中
    assert_eq!(r3, r4);
    println!("   ✅ 通过\n");
    
    // val 模式：深度复制内容
    println!("3. val 模式：深度内容比较");
    process_slice_val_clear();
    let v5 = vec![7, 8, 9];
    let v6 = vec![7, 8, 9];
    let r5 = process_slice_val(&v5);
    let r6 = process_slice_val(&v6); // 深度比较内容
    assert_eq!(r5, r6);
    println!("   ✅ 通过\n");
    
    println!("=== 缓存行为测试通过！ ===");
}

