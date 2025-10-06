use mau::min;

#[test]
fn test_min_macro_basic() {
    let d = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let min_val = min!(|i| d[i], 0..d.len());
    assert_eq!(min_val, 1);
    println!("Basic test passed: min value = {}", min_val);
}

#[test]
fn test_min_macro_partial_range() {
    let d = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let min_val = min!(|i| d[i], 2..6);
    assert_eq!(min_val, 1);
    println!("Partial range test passed: min value = {}", min_val);
}

#[test]
fn test_min_macro_floats() {
    let floats = vec![3.5, 1.2, 4.8, 1.1, 5.9, 2.3];
    let min_float = min!(|i| floats[i], 0..floats.len());
    assert_eq!(min_float, 1.1);
    println!("Float test passed: min value = {}", min_float);
}

#[test]
fn test_min_macro_complex_expression() {
    let data = vec![10, 5, 8, 3, 7];
    let min_val = min!(|i| data[i] * 2 + 1, 0..data.len());
    assert_eq!(min_val, 7); // 3 * 2 + 1 = 7
    println!("Complex expression test passed: min value = {}", min_val);
}

#[test]
fn test_min_macro_strings() {
    let words: Vec<&str> = vec!["apple", "banana", "cherry", "date"];
    let min_word = min!(|i: usize| words[i].len(), 0..words.len());
    assert_eq!(min_word, 4); // "date" has 4 characters
    println!("String test passed: min length = {}", min_word);
}
