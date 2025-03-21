use std::collections::HashMap;

fn main() {
    // HashMap 默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表（hash
    // table）的拒绝服务（Denial of Service, DoS）攻击 NOTE https://en.wikipedia.org/wiki/SipHash
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("{score}");

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        // 注意，移动
        map.insert(field_name, field_value);
        println!("{map:#?}");
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        // 覆盖旧值
        scores.insert(String::from("Blue"), 25);

        println!("{scores:#?}");
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // 如果不存在，就插入
        // entry() 的返回值是一个枚举，它代表了可能存在也可能不存在的值
        // or_insert() 返回这个键的值的一个可变引用
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:#?}");
    }

    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            map.entry(word)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        println!("{map:#?}");
    }
}
