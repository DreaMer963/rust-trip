fn main() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(5);
    v1.push(6);
    let mut v2 = vec![1,2,3];

    let second: &i32 = &v2[2];
    let first = &v2[0]; // 不可变引用
    // v2.push(6);   这是一个mut&
    println!("{}", first);
    match v2.get(4) {
        Some(second) => println!("{}", second),
        None => println!("There is no second element"),
    }

    let mut v3 = vec![5, 6, 7, 8];

    for i in &mut v3 {
        *i += 50;
    }

    for i in &v3 {
        println!("{}", i);
    }

    // 枚举使得可以在vector中存储不同类型的值
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    // hashMap
    // 哈希 map 是同质的：所有的键必须是相同类型，值也必须都是相同类型。
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();

} // vec在离开作用域时会被丢弃
