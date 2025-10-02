use mau::memo;

#[test]
fn test_thoth_queue_transformation() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = f(&nums, 10);
    assert_eq!(result, 55); // 预期结果：所有数字的和
}

#[memo]
fn f(nums: &Vec<i32>, n: usize) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return nums[0];
    }

    let start = n.saturating_sub(4);
    let end = n.saturating_sub(1);
    
    // MauQueue 会被宏转换器转换为循环
    let s = MauQueue(
        move || start,
        move || end,
        |i| {
            let current_value = nums[i];
            let prev_max = f(nums, i);
            std::cmp::max(prev_max, prev_max + current_value)
        }
    );
    s
}
/*
omem
*/
