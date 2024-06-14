use std::{
    fmt::Display,
    ops::{Add, Deref},
};

// 不实用范型的原因在于，如果使用范型，则可以实现该 trait 多次
trait Iterator {
    // 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 运算符重载（Operator overloading）
// Rust 并不允许创建自定义运算符或重载任意运算符，不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// 默认类型参数（default type parameters）
// 如果实现 Add trait 时不指定 Rhs 的具体类型，Rhs 的类型将是默认的 Self 类型，也就是在其上实现 Add 的类型
// 不使用默认类型参数
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// 说明 OutlinePrint 只能用于同时实现了 Display trait 的类型
trait OutlinePrint: Display {
    fn outline_print(&self) {
        let _output = self.to_string();
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// newtype 模式（newtype pattern），绕开孤儿规则（orphan rule）的限制
// 它涉及到在一个元组结构体中创建一个新类型
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 暴露内部类型的每一个方法
impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let millimeters = Millimeters(50);
    let meters = Meters(2);
    let _millimeters = millimeters + meters;

    let person = Human;
    // *waving arms furiously*
    person.fly();

    // 显式
    Pilot::fly(&person);
    Wizard::fly(&person);

    // A baby dog is called a Spot
    println!("A baby dog is called a {}", Dog::baby_name());
    // A baby dog is called a puppy
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("{w}");
}
