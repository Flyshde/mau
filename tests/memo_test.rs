use mau::memo;

#[memo(key = ref )]
fn simple_add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_memo_simple() {
    let result1 = simple_add(1, 2);
    let result2 = simple_add(1, 2);
    assert_eq!(result1, 3);
    assert_eq!(result2, 3);
}
