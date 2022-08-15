use std::collections::HashMap;

fn main() {
    // 使用 SipHash
    // https://en.wikipedia.org/wiki/SipHash
    let mut map = HashMap::new();
    map.insert(String::from("123"), 123);
    println!("{:#?}", map);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let map: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:#?}", map);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // 注意，移动
    // TODO 也可以将将值的引用插入哈希 map，第十章
    map.insert(field_name, field_value);
    println!("{:#?}", map);

    if let Some(value) = map.get("Favorite color") {
        println!("{}", value);
    };
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 替换旧值
    scores.insert(String::from("Blue"), 25);
    // 如果不存在，就插入
    // entry() 的返回值是一个枚举，它代表了可能存在也可能不存在的值
    // or_insert() 返回这个键的值的一个可变引用
    scores.entry(String::from("Blue")).or_insert(30);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
