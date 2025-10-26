//! 演示 each! 宏的用法示例
//!
//! each! 宏用于对指定范围内的每个索引执行闭包

use mau::each;

fn main() {
    // 示例：展示如何使用 each! 宏
    let data = vec![3, 1, 4, 1, 5, 9];
    
    println!("遍历整个数组：");
    each!(|i| {
        println!("data[{}] = {}", i, data[i]);
    }, 0..data.len());
    
    println!("\n遍历部分范围（索引 2 到 4）：");
    each!(|i| {
        println!("data[{}] = {}", i, data[i]);
    }, 2..5);
    
    println!("\n累积求和：");
    let mut sum = 0;
    each!(|i| {
        sum += data[i];
    }, 0..data.len());
    println!("总和: {}", sum);
}

