use mau::memo;

#[memo]
fn F(a: usize, b: usize, k: usize, l: usize, n: usize) -> usize {
    match n {
        0 => k,
        1 => l,
        _ => a * F(a, b, k, l, n - 1) + b * F(a, b, k, l, n - 2),
    }
}

#[test]
fn test_F_function() {
    let result = F(2, 3, 1, 1, 5);
    println!("Result: {}", result);
}
