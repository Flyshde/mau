use mau::memo;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue, // 用于测试，虽然未使用但保留
    RGB(u8, u8, u8),
}

#[test]
fn test_custom_types() {
    println!("=== Testing Custom Types ===");
    
    // 测试 Point 结构体
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 }; // 相同内容，应该命中缓存
    let p3 = Point { x: 1, y: 3 }; // 不同内容，不应该命中缓存
    
    let result1 = custom_point(p1);
    let result2 = custom_point(p2); // 应该命中缓存
    let result3 = custom_point(p3); // 不应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    println!("Point test: {} == {} != {}", result1, result2, result3);
    
    // 测试 Person 结构体
    let p1 = Person { name: "Alice".to_string(), age: 30 };
    let p2 = Person { name: "Alice".to_string(), age: 30 }; // 相同内容，应该命中缓存
    let p3 = Person { name: "Bob".to_string(), age: 30 }; // 不同内容，不应该命中缓存
    
    let result1 = custom_person(p1);
    let result2 = custom_person(p2); // 应该命中缓存
    let result3 = custom_person(p3); // 不应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    println!("Person test: {} == {} != {}", result1, result2, result3);
    
    // 测试 Color 枚举
    let c1 = Color::Red;
    let c2 = Color::Red; // 相同内容，应该命中缓存
    let c3 = Color::Green; // 不同内容，不应该命中缓存
    let c4 = Color::RGB(255, 0, 0);
    let c5 = Color::RGB(255, 0, 0); // 相同内容，应该命中缓存
    
    let result1 = custom_color(c1);
    let result2 = custom_color(c2); // 应该命中缓存
    let result3 = custom_color(c3); // 不应该命中缓存
    let result4 = custom_color(c4);
    let result5 = custom_color(c5); // 应该命中缓存
    
    assert_eq!(result1, result2);
    assert_ne!(result1, result3);
    assert_eq!(result4, result5);
    println!("Color test: {} == {} != {}, {} == {}", result1, result2, result3, result4, result5);
    
    // 测试 Vec<Point> 类型
    let vec1 = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    let vec2 = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }]; // 相同内容，应该命中缓存
    
    let result1 = custom_vec_point(vec1);
    let result2 = custom_vec_point(vec2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Vec<Point> test: {} == {}", result1, result2);
    
    // 测试 BTreeMap<String, Point> 类型
    let mut map1 = BTreeMap::new();
    map1.insert("origin".to_string(), Point { x: 0, y: 0 });
    map1.insert("center".to_string(), Point { x: 5, y: 5 });
    
    let mut map2 = BTreeMap::new();
    map2.insert("origin".to_string(), Point { x: 0, y: 0 });
    map2.insert("center".to_string(), Point { x: 5, y: 5 }); // 相同内容，应该命中缓存
    
    let result1 = custom_btreemap_point(map1);
    let result2 = custom_btreemap_point(map2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("BTreeMap<String, Point> test: {} == {}", result1, result2);
    
    // 测试混合自定义类型参数
    let p1 = Point { x: 1, y: 2 };
    let c1 = Color::Red;
    let p2 = Point { x: 1, y: 2 };
    let c2 = Color::Red;
    
    let result1 = custom_mixed(p1, c1);
    let result2 = custom_mixed(p2, c2); // 应该命中缓存
    
    assert_eq!(result1, result2);
    println!("Mixed custom types test: {} == {}", result1, result2);
}

#[memo]
fn custom_point(p: Point) -> i32 {
    println!("Computing custom_point({:?})", p);
    p.x + p.y
}

#[memo]
fn custom_person(p: Person) -> usize {
    println!("Computing custom_person({:?})", p);
    p.name.len() + p.age as usize
}

#[memo]
fn custom_color(c: Color) -> u32 {
    println!("Computing custom_color({:?})", c);
    match c {
        Color::Red => 0xFF0000,
        Color::Green => 0x00FF00,
        Color::Blue => 0x0000FF,
        Color::RGB(r, g, b) => ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
    }
}

#[memo]
fn custom_vec_point(vec: Vec<Point>) -> i32 {
    println!("Computing custom_vec_point({:?})", vec);
    vec.iter().map(|p| p.x + p.y).sum()
}

#[memo]
fn custom_btreemap_point(map: BTreeMap<String, Point>) -> i32 {
    println!("Computing custom_btreemap_point({:?})", map);
    map.values().map(|p| p.x + p.y).sum()
}

#[memo]
fn custom_mixed(p: Point, c: Color) -> String {
    println!("Computing custom_mixed({:?}, {:?})", p, c);
    format!("Point({},{}) with {:?}", p.x, p.y, c)
}
