use mau::memo;
use std::sync::atomic::{AtomicUsize, Ordering};

static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// 测试 ref 模式：先比地址，若地址相等直接返回；否则再比内容
#[memo(key=ref)]
fn process_array(data: &[i32]) -> i32 {
    CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    data.iter().sum()
}

#[test]
fn test_ref_same_address() {
    process_array_clear();  // 清空缓存
    CALL_COUNT.store(0, Ordering::SeqCst);
    
    let arr = vec![1, 2, 3, 4, 5];
    
    // 第1次调用
    let result1 = process_array(&arr);
    assert_eq!(result1, 15);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第2次调用（相同地址）
    let result2 = process_array(&arr);
    assert_eq!(result2, 15);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1, "相同地址应该直接返回，不重新计算");
}

#[test]
fn test_ref_different_address_same_content() {
    process_array_clear();  // 清空缓存
    CALL_COUNT.store(0, Ordering::SeqCst);
    
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![1, 2, 3, 4, 5];  // 不同地址，相同内容
    
    // 第1次调用
    let result1 = process_array(&arr1);
    assert_eq!(result1, 15);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第2次调用（不同地址，但内容相同）
    let result2 = process_array(&arr2);
    assert_eq!(result2, 15);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1, "内容相同应该命中缓存");
}

#[test]
fn test_ref_different_content() {
    process_array_clear();  // 清空缓存
    CALL_COUNT.store(0, Ordering::SeqCst);
    
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![5, 4, 3, 2, 1];  // 不同内容
    
    // 第1次调用
    let result1 = process_array(&arr1);
    assert_eq!(result1, 15);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第2次调用（内容不同）
    let result2 = process_array(&arr2);
    assert_eq!(result2, 15);
    assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2, "内容不同应该重新计算");
}

#[test]
fn test_ref_key_without_r() {
    // 测试 key=ref 可以直接使用，不需要 r#ref
    
    #[memo(key=ref)]
    fn simple_calc(n: i32) -> i32 {
        n * n
    }
    
    assert_eq!(simple_calc(5), 25);
    assert_eq!(simple_calc(5), 25);  // 缓存命中
}

