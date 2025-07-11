fn main() {
    let v = [1, 2, 3];

    // 迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑
    // 迭代器是惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果
    // 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait
    // 还有 into_iter() / iter_mut()，获取所有权/可变引用
    // 在迭代器上调用 next 方法会改变迭代器内部的状态
    let mut v_iter = v.iter();

    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);

    // 消费适配器（consuming adaptors）
    // 注意 sum() 会获取所有权
    let sum: i32 = v.iter().sum();
    println!("{sum}");

    // 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{v2:?}");

    let v2: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{v2:?}");
}
