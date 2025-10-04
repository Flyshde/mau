use mau::memo;
use std::collections::BTreeMap;

#[test]
fn test_cache_key_uses_values_not_addresses() {
    println!("=== Testing Cache Keys Use Values, Not Addresses ===");

    // --- 测试1: 相同内容的不同内存位置应该命中缓存 ---
    println!("\n--- Test 1: Same Content, Different Memory Locations ---");
    
    // 创建相同内容但不同内存位置的数组
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![1, 2, 3, 4, 5]; // 相同内容，不同内存位置
    let arr3 = vec![6, 7, 8, 9, 10]; // 不同内容
    
    let result1 = sum_array(&arr1);
    let result2 = sum_array(&arr2);
    let result3 = sum_array(&arr3);
    
    assert_eq!(result1, result2); // 相同内容应该命中缓存
    assert_ne!(result1, result3); // 不同内容应该得到不同结果
    
    println!("Array1 sum: {} (first call)", result1);
    println!("Array2 sum: {} (should hit cache)", result2);
    println!("Array3 sum: {} (different content)", result3);

    // --- 测试2: 多维数组的相同内容测试 ---
    println!("\n--- Test 2: Multi-Dimensional Arrays with Same Content ---");
    
    let matrix1 = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let matrix2 = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];
    let matrix3 = [
        [10, 20, 30],
        [40, 50, 60],
        [70, 80, 90]
    ];
    
    let sum1 = sum_2d_matrix(&matrix1);
    let sum2 = sum_2d_matrix(&matrix2);
    let sum3 = sum_2d_matrix(&matrix3);
    
    assert_eq!(sum1, sum2); // 相同内容应该命中缓存
    assert_ne!(sum1, sum3); // 不同内容应该得到不同结果
    
    println!("Matrix1 sum: {} (first call)", sum1);
    println!("Matrix2 sum: {} (should hit cache)", sum2);
    println!("Matrix3 sum: {} (different content)", sum3);

    // --- 测试3: f64数组的位模式转换测试 ---
    println!("\n--- Test 3: f64 Arrays with Bit Pattern Conversion ---");
    
    let floats1 = [1.0, 2.0, 3.0, 4.0];
    let floats2 = [1.0, 2.0, 3.0, 4.0]; // 相同内容
    let floats3 = [10.0, 20.0, 30.0, 40.0]; // 不同内容
    
    let avg1 = average_floats(&floats1);
    let avg2 = average_floats(&floats2);
    let avg3 = average_floats(&floats3);
    
    assert_eq!(avg1, avg2); // 相同内容应该命中缓存
    assert_ne!(avg1, avg3); // 不同内容应该得到不同结果
    
    println!("Floats1 average: {} (first call)", avg1);
    println!("Floats2 average: {} (should hit cache)", avg2);
    println!("Floats3 average: {} (different content)", avg3);

    // --- 测试4: 字符数组的相同内容测试 ---
    println!("\n--- Test 4: Character Arrays with Same Content ---");
    
    let chars1 = [['A', 'B', 'C'], ['D', 'E', 'F']];
    let chars2 = [['A', 'B', 'C'], ['D', 'E', 'F']]; // 相同内容
    let chars3 = [['X', 'Y', 'Z'], ['1', '2', '3'], ['4', '5', '6']]; // 不同内容（更多行）
    
    let count1 = count_char_arrays(&chars1);
    let count2 = count_char_arrays(&chars2);
    let count3 = count_char_arrays(&chars3);
    
    assert_eq!(count1, count2); // 相同内容应该命中缓存
    assert_ne!(count1, count3); // 不同内容应该得到不同结果
    
    println!("Chars1 count: {} (first call)", count1);
    println!("Chars2 count: {} (should hit cache)", count2);
    println!("Chars3 count: {} (different content)", count3);

    // --- 测试5: 复杂嵌套结构的相同内容测试 ---
    println!("\n--- Test 5: Complex Nested Structures ---");
    
    let complex1 = create_complex_data();
    let complex2 = create_complex_data(); // 相同内容，不同内存位置
    let complex3 = create_different_complex_data(); // 不同内容
    
    let result1 = process_complex_data(&complex1);
    let result2 = process_complex_data(&complex2);
    let result3 = process_complex_data(&complex3);
    
    assert_eq!(result1, result2); // 相同内容应该命中缓存
    assert_ne!(result1, result3); // 不同内容应该得到不同结果
    
    println!("Complex1 result: {} (first call)", result1);
    println!("Complex2 result: {} (should hit cache)", result2);
    println!("Complex3 result: {} (different content)", result3);

    // --- 测试6: 验证缓存确实被使用 ---
    println!("\n--- Test 6: Verify Cache is Actually Used ---");
    
    // 重置计数器
    reset_call_count();
    
    // 第一次调用
    let _ = sum_array(&vec![100, 200, 300]);
    let first_call_count = get_call_count();
    
    // 第二次调用相同内容
    let _ = sum_array(&vec![100, 200, 300]);
    let second_call_count = get_call_count();
    
    // 第三次调用不同内容
    let _ = sum_array(&vec![400, 500, 600]);
    let third_call_count = get_call_count();
    
    println!("First call count: {}", first_call_count);
    println!("Second call count: {} (should be same as first - cache hit)", second_call_count);
    println!("Third call count: {} (should be higher - cache miss)", third_call_count);
    
    // 第二次调用应该命中缓存，所以调用次数不应该增加
    assert_eq!(first_call_count, second_call_count);
    // 第三次调用不同内容，应该增加调用次数
    assert!(third_call_count > second_call_count);

    println!("\n=== All Cache Key Value Tests Passed! ===");
}

// 全局调用计数器
static mut CALL_COUNT: u32 = 0;

fn reset_call_count() {
    unsafe { CALL_COUNT = 0; }
}

fn get_call_count() -> u32 {
    unsafe { CALL_COUNT }
}

fn increment_call_count() {
    unsafe { CALL_COUNT += 1; }
}

// 测试函数：数组求和
#[memo]
fn sum_array(arr: &[i32]) -> i32 {
    increment_call_count();
    println!("Computing sum_array({:?}) - Call #{}", arr, get_call_count());
    arr.iter().sum()
}

// 测试函数：2D矩阵求和
#[memo]
fn sum_2d_matrix(matrix: &[[i32; 3]]) -> i32 {
    println!("Computing sum_2d_matrix({:?})", matrix);
    matrix.iter().map(|row| row.iter().sum::<i32>()).sum()
}

// 测试函数：浮点数数组平均值
#[memo]
fn average_floats(floats: &[f64]) -> f64 {
    println!("Computing average_floats({:?})", floats);
    if floats.is_empty() {
        0.0
    } else {
        floats.iter().sum::<f64>() / floats.len() as f64
    }
}

// 测试函数：字符数组计数
#[memo]
fn count_char_arrays(chars: &[[char; 3]]) -> usize {
    println!("Computing count_char_arrays({:?})", chars);
    chars.len() * 3 // 每个子数组有3个字符
}

// 复杂数据结构
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ComplexData {
    numbers: Vec<i32>,
    matrix: Vec<Vec<i32>>,
    metadata: BTreeMap<String, i32>,
}

// 测试函数：处理复杂数据
#[memo]
fn process_complex_data(data: &ComplexData) -> i32 {
    println!("Computing process_complex_data({:?})", data);
    let number_sum: i32 = data.numbers.iter().sum();
    let matrix_sum: i32 = data.matrix.iter()
        .map(|row| row.iter().sum::<i32>())
        .sum();
    let metadata_sum: i32 = data.metadata.values().sum();
    
    number_sum + matrix_sum + metadata_sum
}

// 创建复杂数据
fn create_complex_data() -> ComplexData {
    let mut metadata = BTreeMap::new();
    metadata.insert("key1".to_string(), 10);
    metadata.insert("key2".to_string(), 20);
    
    ComplexData {
        numbers: vec![1, 2, 3, 4, 5],
        matrix: vec![vec![1, 2], vec![3, 4]],
        metadata,
    }
}

// 创建不同的复杂数据
fn create_different_complex_data() -> ComplexData {
    let mut metadata = BTreeMap::new();
    metadata.insert("key1".to_string(), 100);
    metadata.insert("key2".to_string(), 200);
    
    ComplexData {
        numbers: vec![10, 20, 30, 40, 50],
        matrix: vec![vec![10, 20], vec![30, 40]],
        metadata,
    }
}
