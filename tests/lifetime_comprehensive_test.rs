use mau::memo;
use std::sync::atomic::{AtomicUsize, Ordering};

// ========== 测试 lifetime=problem (默认行为) ==========

static PROBLEM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[memo(lifetime=problem)]
fn calc_problem(n: i32) -> i32 {
    PROBLEM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

#[test]
fn test_lifetime_problem_clears_cache() {
    PROBLEM_CALL_COUNT.store(0, Ordering::SeqCst);
    
    // 第一次调用
    let result1 = calc_problem_start(5);
    assert_eq!(result1, 25);
    assert_eq!(PROBLEM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第二次调用相同参数 - lifetime=problem 应该清除缓存
    let result2 = calc_problem_start(5);
    assert_eq!(result2, 25);
    assert_eq!(PROBLEM_CALL_COUNT.load(Ordering::SeqCst), 2, "problem 模式应该清除缓存");
}

// ========== 测试 lifetime=program + key=ptr (应该清除缓存) ==========

static PTR_PROGRAM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[memo(key=ptr, lifetime=program)]
fn calc_ptr_program(data: &[i32]) -> i32 {
    PTR_PROGRAM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    data.iter().sum()
}

#[test]
fn test_lifetime_program_with_ptr_clears_cache() {
    PTR_PROGRAM_CALL_COUNT.store(0, Ordering::SeqCst);
    
    let arr = vec![1, 2, 3, 4, 5];
    
    // 第一次调用
    let result1 = calc_ptr_program_start(&arr);
    assert_eq!(result1, 15);
    assert_eq!(PTR_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第二次调用相同数组 - lifetime=program + key=ptr 应该清除缓存
    // 因为键包含地址，旧地址的缓存无法被新问题访问，只会浪费内存
    let result2 = calc_ptr_program_start(&arr);
    assert_eq!(result2, 15);
    assert_eq!(PTR_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 2, "program+ptr 模式应该清除缓存");
    
    // 清除缓存是自动的，手动清除也可以
    calc_ptr_program_clear();
    let result3 = calc_ptr_program_start(&arr);
    assert_eq!(result3, 15);
    assert_eq!(PTR_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 3, "清除后应该重新计算");
}

#[test]
fn test_lifetime_program_ptr_different_arrays() {
    PTR_PROGRAM_CALL_COUNT.store(0, Ordering::SeqCst);
    calc_ptr_program_clear();
    
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![1, 2, 3]; // 相同内容但不同地址
    
    // 第一次调用 arr1
    let result1 = calc_ptr_program_start(&arr1);
    assert_eq!(result1, 6);
    assert_eq!(PTR_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 调用 arr2（不同地址）- 因为使用地址作为键，应该重新计算
    let result2 = calc_ptr_program_start(&arr2);
    assert_eq!(result2, 6);
    assert_eq!(PTR_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 2, "不同地址应该重新计算");
    
    // 再次调用 arr1 - 因为 program+ptr 每次都清除，应该重新计算
    let result3 = calc_ptr_program_start(&arr1);
    assert_eq!(result3, 6);
    assert_eq!(PTR_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 3, "program+ptr 每次都清除缓存");
}

// ========== 测试 lifetime=program + key=ref (应该清除缓存) ==========

static REF_PROGRAM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[memo(key=ref, lifetime=program)]
fn calc_ref_program(data: &[i32]) -> i32 {
    REF_PROGRAM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    data.iter().sum()
}

#[test]
fn test_lifetime_program_with_ref_clears_cache() {
    REF_PROGRAM_CALL_COUNT.store(0, Ordering::SeqCst);
    calc_ref_program_clear();
    
    let arr = vec![1, 2, 3, 4, 5];
    
    // 第一次调用
    let result1 = calc_ref_program_start(&arr);
    assert_eq!(result1, 15);
    assert_eq!(REF_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第二次调用相同数组 - lifetime=program + key=ref 应该清除缓存
    // 因为键包含地址，旧地址的缓存无法被新问题访问
    let result2 = calc_ref_program_start(&arr);
    assert_eq!(result2, 15);
    assert_eq!(REF_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 2, "program+ref 模式应该清除缓存");
}

#[test]
fn test_lifetime_program_ref_different_arrays_same_content() {
    REF_PROGRAM_CALL_COUNT.store(0, Ordering::SeqCst);
    calc_ref_program_clear();
    
    let arr1 = vec![1, 2, 3];
    let arr2 = vec![1, 2, 3]; // 相同内容但不同地址
    
    // 第一次调用 arr1
    let result1 = calc_ref_program_start(&arr1);
    assert_eq!(result1, 6);
    assert_eq!(REF_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 调用 arr2 - program+ref 模式会清除缓存，应该重新计算
    let result2 = calc_ref_program_start(&arr2);
    assert_eq!(result2, 6);
    assert_eq!(REF_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 2, "program+ref 模式清除缓存，重新计算");
}

// ========== 测试 lifetime=program + key=val (应该保留缓存) ==========

static VAL_PROGRAM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[memo(key=val, lifetime=program)]
fn calc_val_program(data: &[i32]) -> i32 {
    VAL_PROGRAM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    data.iter().sum()
}

#[test]
fn test_lifetime_program_with_val_keeps_cache() {
    VAL_PROGRAM_CALL_COUNT.store(0, Ordering::SeqCst);
    
    let arr = vec![1, 2, 3, 4, 5];
    
    // 第一次调用
    let result1 = calc_val_program_start(&arr);
    assert_eq!(result1, 15);
    assert_eq!(VAL_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第二次调用 - lifetime=program + key=val 应该保留缓存
    // 因为键完全基于值，相同输入可以跨问题复用
    let result2 = calc_val_program_start(&arr);
    assert_eq!(result2, 15);
    assert_eq!(VAL_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1, 
        "val 模式 + program 应该保留缓存（键基于值）");
}

// ========== 测试递归函数的 lifetime 行为 ==========

static FIB_PROBLEM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static FIB_PROGRAM_PTR_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[memo(lifetime=problem)]
fn fib_problem(n: i32) -> i32 {
    FIB_PROBLEM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    if n <= 1 {
        n
    } else {
        fib_problem(n - 1) + fib_problem(n - 2)
    }
}

#[memo(key=ptr, lifetime=program)]
fn fib_program_ptr(n: i32) -> i32 {
    FIB_PROGRAM_PTR_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    if n <= 1 {
        n
    } else {
        fib_program_ptr(n - 1) + fib_program_ptr(n - 2)
    }
}

#[test]
fn test_recursive_lifetime_problem() {
    FIB_PROBLEM_CALL_COUNT.store(0, Ordering::SeqCst);
    
    // 第一次计算 fib(5)
    let result1 = fib_problem_start(5);
    assert_eq!(result1, 5);
    let count1 = FIB_PROBLEM_CALL_COUNT.load(Ordering::SeqCst);
    assert!(count1 > 0);
    
    // 第二次计算 fib(5) - problem 模式清除缓存，应该重新计算相同次数
    FIB_PROBLEM_CALL_COUNT.store(0, Ordering::SeqCst);
    let result2 = fib_problem_start(5);
    assert_eq!(result2, 5);
    let count2 = FIB_PROBLEM_CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(count1, count2, "problem 模式每次都应该重新计算");
}

#[test]
fn test_recursive_lifetime_program_ptr() {
    FIB_PROGRAM_PTR_CALL_COUNT.store(0, Ordering::SeqCst);
    
    // 第一次计算 fib(5)
    let result1 = fib_program_ptr_start(5);
    assert_eq!(result1, 5);
    let count1 = FIB_PROGRAM_PTR_CALL_COUNT.load(Ordering::SeqCst);
    assert!(count1 > 0);
    
    // 第二次计算 fib(5) - 因为参数不是引用，键中不包含地址，应该保留缓存
    FIB_PROGRAM_PTR_CALL_COUNT.store(0, Ordering::SeqCst);
    let result2 = fib_program_ptr_start(5);
    assert_eq!(result2, 5);
    let count2 = FIB_PROGRAM_PTR_CALL_COUNT.load(Ordering::SeqCst);
    assert_eq!(0, count2, "无引用参数时，program+ptr 应该保留缓存，不重新计算");
}

// ========== 测试多线程下的 lifetime 行为 ==========

static MULTI_PROBLEM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static MULTI_PROGRAM_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[memo(thread=multi, lifetime=problem)]
fn calc_multi_problem(n: i32) -> i32 {
    MULTI_PROBLEM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

// 注意：参数是 i32，不是引用，所以键中不包含地址
#[memo(thread=multi, key=ptr, lifetime=program)]
fn calc_multi_program(n: i32) -> i32 {
    MULTI_PROGRAM_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

#[test]
fn test_multithread_lifetime_problem() {
    MULTI_PROBLEM_CALL_COUNT.store(0, Ordering::SeqCst);
    
    let result1 = calc_multi_problem_start(10);
    assert_eq!(result1, 100);
    assert_eq!(MULTI_PROBLEM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    let result2 = calc_multi_problem_start(10);
    assert_eq!(result2, 100);
    assert_eq!(MULTI_PROBLEM_CALL_COUNT.load(Ordering::SeqCst), 2, "多线程 problem 模式应该清除缓存");
}

#[test]
fn test_multithread_lifetime_program() {
    MULTI_PROGRAM_CALL_COUNT.store(0, Ordering::SeqCst);
    
    let result1 = calc_multi_program_start(10);
    assert_eq!(result1, 100);
    assert_eq!(MULTI_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    let result2 = calc_multi_program_start(10);
    assert_eq!(result2, 100);
    assert_eq!(MULTI_PROGRAM_CALL_COUNT.load(Ordering::SeqCst), 1, "无引用参数时，多线程 program+ptr 应该保留缓存");
}

// ========== 测试手动清除缓存 ==========

static MANUAL_CLEAR_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// 参数是 i32，不是引用，所以键中不包含地址
#[memo(key=ptr, lifetime=program)]
fn calc_manual_clear(n: i32) -> i32 {
    MANUAL_CLEAR_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

#[test]
fn test_manual_cache_clear() {
    MANUAL_CLEAR_CALL_COUNT.store(0, Ordering::SeqCst);
    
    // 第一次调用
    let result1 = calc_manual_clear_start(5);
    assert_eq!(result1, 25);
    assert_eq!(MANUAL_CLEAR_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    // 第二次调用 - 无引用参数，应该保留缓存
    let result2 = calc_manual_clear_start(5);
    assert_eq!(result2, 25);
    assert_eq!(MANUAL_CLEAR_CALL_COUNT.load(Ordering::SeqCst), 1, "无引用参数时应该保留缓存");
    
    // 手动调用 clear 清除缓存
    calc_manual_clear_clear();
    
    // 第三次调用 - 缓存被清除，重新计算
    let result3 = calc_manual_clear_start(5);
    assert_eq!(result3, 25);
    assert_eq!(MANUAL_CLEAR_CALL_COUNT.load(Ordering::SeqCst), 2);
}

// ========== 测试没有引用参数的函数（键中不包含地址） ==========

static NO_REF_PROGRAM_PTR_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static NO_REF_PROGRAM_REF_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static NO_REF_PROGRAM_VAL_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);

// 没有引用参数的函数，设置 lifetime=program 应该保留缓存
// 因为键中不包含地址信息，完全基于值
#[memo(key=ptr, lifetime=program)]
fn calc_no_ref_program_ptr(n: i32) -> i32 {
    NO_REF_PROGRAM_PTR_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

#[memo(key=ref, lifetime=program)]
fn calc_no_ref_program_ref(n: i32) -> i32 {
    NO_REF_PROGRAM_REF_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

#[memo(key=val, lifetime=program)]
fn calc_no_ref_program_val(n: i32) -> i32 {
    NO_REF_PROGRAM_VAL_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    n * n
}

#[test]
fn test_lifetime_program_no_ref_params_keeps_cache() {
    // 测试 key=ptr + 无引用参数
    NO_REF_PROGRAM_PTR_CALL_COUNT.store(0, Ordering::SeqCst);
    let result1 = calc_no_ref_program_ptr_start(5);
    assert_eq!(result1, 25);
    assert_eq!(NO_REF_PROGRAM_PTR_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    let result2 = calc_no_ref_program_ptr_start(5);
    assert_eq!(result2, 25);
    assert_eq!(NO_REF_PROGRAM_PTR_CALL_COUNT.load(Ordering::SeqCst), 1, 
        "无引用参数时，key=ptr + lifetime=program 应该保留缓存");
    
    // 测试 key=ref + 无引用参数
    NO_REF_PROGRAM_REF_CALL_COUNT.store(0, Ordering::SeqCst);
    let result3 = calc_no_ref_program_ref_start(5);
    assert_eq!(result3, 25);
    assert_eq!(NO_REF_PROGRAM_REF_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    let result4 = calc_no_ref_program_ref_start(5);
    assert_eq!(result4, 25);
    assert_eq!(NO_REF_PROGRAM_REF_CALL_COUNT.load(Ordering::SeqCst), 1, 
        "无引用参数时，key=ref + lifetime=program 应该保留缓存");
    
    // 测试 key=val + 无引用参数
    NO_REF_PROGRAM_VAL_CALL_COUNT.store(0, Ordering::SeqCst);
    let result5 = calc_no_ref_program_val_start(5);
    assert_eq!(result5, 25);
    assert_eq!(NO_REF_PROGRAM_VAL_CALL_COUNT.load(Ordering::SeqCst), 1);
    
    let result6 = calc_no_ref_program_val_start(5);
    assert_eq!(result6, 25);
    assert_eq!(NO_REF_PROGRAM_VAL_CALL_COUNT.load(Ordering::SeqCst), 1, 
        "无引用参数时，key=val + lifetime=program 应该保留缓存");
}

// ========== 测试不同 key 模式的组合 ==========

#[test]
fn test_lifetime_combinations() {
    // problem + ptr
    static COUNT1: AtomicUsize = AtomicUsize::new(0);
    #[memo(key=ptr, lifetime=problem)]
    fn f1(n: i32) -> i32 {
        COUNT1.fetch_add(1, Ordering::SeqCst);
        n
    }
    
    COUNT1.store(0, Ordering::SeqCst);
    f1_start(5);
    f1_start(5);
    assert_eq!(COUNT1.load(Ordering::SeqCst), 2, "problem 模式应该清除");
    
    // problem + ref
    static COUNT2: AtomicUsize = AtomicUsize::new(0);
    #[memo(key=ref, lifetime=problem)]
    fn f2(n: i32) -> i32 {
        COUNT2.fetch_add(1, Ordering::SeqCst);
        n
    }
    
    COUNT2.store(0, Ordering::SeqCst);
    f2_start(5);
    f2_start(5);
    assert_eq!(COUNT2.load(Ordering::SeqCst), 2, "problem 模式应该清除");
    
    // problem + val
    static COUNT3: AtomicUsize = AtomicUsize::new(0);
    #[memo(key=val, lifetime=problem)]
    fn f3(n: i32) -> i32 {
        COUNT3.fetch_add(1, Ordering::SeqCst);
        n
    }
    
    COUNT3.store(0, Ordering::SeqCst);
    f3_start(5);
    f3_start(5);
    assert_eq!(COUNT3.load(Ordering::SeqCst), 2, "problem 模式应该清除");
    
    // program + ptr (但无引用参数)
    static COUNT4: AtomicUsize = AtomicUsize::new(0);
    #[memo(key=ptr, lifetime=program)]
    fn f4(n: i32) -> i32 {
        COUNT4.fetch_add(1, Ordering::SeqCst);
        n
    }
    
    COUNT4.store(0, Ordering::SeqCst);
    f4_start(5);
    f4_start(5);
    assert_eq!(COUNT4.load(Ordering::SeqCst), 1, "无引用参数时，program+ptr 应该保留");
    
    // program + ref (但无引用参数)
    static COUNT5: AtomicUsize = AtomicUsize::new(0);
    #[memo(key=ref, lifetime=program)]
    fn f5(n: i32) -> i32 {
        COUNT5.fetch_add(1, Ordering::SeqCst);
        n
    }
    
    COUNT5.store(0, Ordering::SeqCst);
    f5_start(5);
    f5_start(5);
    assert_eq!(COUNT5.load(Ordering::SeqCst), 1, "无引用参数时，program+ref 应该保留");
    
    // program + val (无引用参数)
    static COUNT6: AtomicUsize = AtomicUsize::new(0);
    #[memo(key=val, lifetime=program)]
    fn f6(n: i32) -> i32 {
        COUNT6.fetch_add(1, Ordering::SeqCst);
        n
    }
    
    COUNT6.store(0, Ordering::SeqCst);
    f6_start(5);
    f6_start(5);
    assert_eq!(COUNT6.load(Ordering::SeqCst), 1, "无引用参数时，program+val 应该保留（键基于值）");
}

