use std::fmt::Display;

// trait 必须和类型一起引入作用域以便使用额外的 trait 方法
// 不能为外部类型实现外部 trait
// 这条规则确保了其他人编写的代码不会破坏你代码
// 没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // 默认实现
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct NewsArticle {
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

// trait bound
fn _notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 任何实现了指定 trait 的类型，是 trait bound 的语法糖
fn _notify2(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// where 从句
fn _notify3<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news! {}", item.summarize());
}

// 返回实现了 trait 的类型
// 注意只能返回单一类型
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // 这里 Self 是 Pair<T>
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 使用 trait bound 有条件地实现方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// blanket implementations
// 对实现了特定 trait 的类型有条件地实现 trait
trait MyTrait {}

impl<T> MyTrait for T where T: Display {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    println!("1 new tweet: {}", tweet.summarize());

    let pair = Pair::new(1, 2);
    pair.cmp_display();
}
