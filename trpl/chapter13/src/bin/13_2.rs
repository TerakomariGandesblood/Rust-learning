fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    // 迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果
    // 迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait
    // 还有 into_iter() / iter_mut()，获取所有权/可变引用
    let iter = v.iter();

    // for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变
    for val in iter {
        println!("{val}");
    }

    // 消费适配器（consuming adaptors）
    // 注意 sum() 会获取所有权
    let sum: i32 = v.iter().sum();
    println!("{sum}");

    // 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    println!("{v2:?}");

    let v2: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{v2:?}");

    // 注意 zip 只产生四对值，zip 在任一输入迭代器返回 None 时也返回 None
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // 关联类型（associated type）
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
