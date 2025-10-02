use mau::memo;

#[test]
fn test_memo_performance() {
    // 测试数据 - 使用用户提供的输入
    let test_cases = vec![
        vec![7.0, 9.0, 6.0, 8.0, 8.0, 4.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![5.0, 4.0, 3.0, 2.0, 1.0],
        vec![1.0, 3.0, 2.0, 4.0, 5.0, 2.0, 1.0],
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    ];

    for (i, seq) in test_cases.iter().enumerate() {
        println!("=== Test Case {} ===", i + 1);
        println!("Sequence: {:?}", seq);
        
        // 测试不使用 memo 的版本
        let start = std::time::Instant::now();
        let result_no_memo = problem_no_memo(&1.0, seq);
        let time_no_memo = start.elapsed();
        
        // 测试使用 memo 的版本
        let start = std::time::Instant::now();
        let result_memo = problem_memo(&1.0, seq);
        let time_memo = start.elapsed();
        
        println!("Result (no memo): {} in {:?}", result_no_memo, time_no_memo);
        println!("Result (with memo): {} in {:?}", result_memo, time_memo);
        
        if result_no_memo == result_memo {
            println!("✅ Results match!");
        } else {
            println!("❌ Results don't match!");
        }
        
        if time_no_memo > time_memo {
            let speedup = time_no_memo.as_nanos() as f64 / time_memo.as_nanos() as f64;
            println!("🚀 Memo speedup: {:.2}x", speedup);
        } else {
            println!("📊 Memo overhead: {:.2}x", time_memo.as_nanos() as f64 / time_no_memo.as_nanos() as f64);
        }
        println!();
    }
}

// 不使用 memo 的版本
fn problem_no_memo(deg: &f64, seq: &[f64]) -> usize {
    match seq.len() {
        0 => 0,
        _ => match *deg >= seq[0] {
            true  => {
                let a = 1 + problem_no_memo(&seq[0], &seq[1..]);
                let b = problem_no_memo(deg, &seq[1..]);
                usize::max(a, b)
            },
            false => problem_no_memo(deg, &seq[1..])
        }
    }
}

// 使用 memo 的版本
#[memo]
fn problem_memo(deg: &f64, seq: &[f64]) -> usize {
    match seq.len() {
        0 => 0,
        _ => match *deg >= seq[0] {
            true  => {
                let a = 1 + problem_memo(&seq[0], &seq[1..]);
                let b = problem_memo(deg, &seq[1..]);
                usize::max(a, b)
            },
            false => problem_memo(deg, &seq[1..])
        }
    }
}