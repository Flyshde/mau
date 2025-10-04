use mau::memo;

#[memo]
fn f(a: usize, b: usize, k: usize, l: usize, n: usize) -> usize {
    match n {
        0 => k,
        1 => l,
        _ => a * f(a, b, k, l, n - 1) + b * f(a, b, k, l, n - 2),
    }
}

#[test]
fn test_f_function() {
    let result = f(2, 3, 1, 1, 5);
    println!("Result: {}", result);
}
