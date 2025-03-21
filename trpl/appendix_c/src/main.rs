// 可派生的 trait

// Debug trait 用于开启格式化字符串中的调试格式，其通过在 {} 占位符中增加 :? 表明
// 在使用 assert_eq! 宏时，Debug trait 是必须的

// PartialEq trait，可以使用 == 和 != 运算符，实现 eq 方法
// 在使用 assert_eq! 宏时，PartialEq trait 是必须的
// Eq trait 表明每一个被标记类型的值等于其自身（例如两个 NaN 互不相等），只能应用于那些实现了
// PartialEq 的类型 对于 HashMap<K, V> 中的 key 来说，Eq 是必须的

// PartialOrd trait，可以使用 <、 >、<= 和 >= 运算符，实现 partial_cmp 方法，只能在实现了 PartialEq
// 的类型上实现 Ord trait 表明任意两个值存在有效顺序，实现了 cmp 方法（例如两个 NaN
// 无法产生顺序），只可以在实现了 PartialOrd 和 Eq 的类型上实现 Ord trait 在 BTreeSet<T>上存值时，
// Ord 是必须的

// Hash trait 实现了 hash 方法
// 在 HashMap<K, V> 上存储数据，存放 key 的时候，Hash 是必须的

// Copy、CLone trait
// 一个实现了 Copy 的类型必须也实现了 Clone

// Default trait 可以创建一个类型的默认值，实现了 default 函数
// 通常结合结构体更新语法一起使用（..Default::default()）

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 4, y: 2 };
    println!("{p1:#?}");
    println!("x = {}, y = {}", p1.x, p1.y);

    let p2 = Point { x: 4, y: 2 };
    println!("{}", p1 == p2);
    println!("{}", p1.eq(&p1));

    let mut hash_map = HashMap::new();
    hash_map.insert(p1, 2);
    println!("{hash_map:#?}");
}
