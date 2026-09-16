// 可自动实现的特性 Auto Traits
// 若给定类型的成员都实现了该特性，那么该类型就隐式地自动实现该特性
// 如：unsafe auto trait Send {}

// 所有的特性都具有隐式的 ?Sized 约束
// trait Trait: ?Sized {}

use std::any::Any;
use std::borrow::Cow;
use std::num::ParseIntError;
use std::str::FromStr;

fn map_any(mut any: Box<dyn Any>) -> Box<dyn Any> {
    if let Some(num) = any.downcast_mut::<i32>() {
        *num += 1;
    } else if let Some(string) = any.downcast_mut::<String>() {
        *string += "!";
    }

    any
}

// 实现 From<T> 特性的类型允许我们从 T 类型转换到自身的类型 Self
// 实现 Into<T> 特性的类型允许我们从自身的类型 Self 转换到 T 类型
// 不能手动实现 Into<T> 特性

// TryFrom 和 TryInto 是可能失败版本的 From 和 Into
// 实现 FromStr 特性的类型允许可失败地从 &str 转换至 Self。使用这一特性的惯用用方式是，调用 &str 实例的 .parse() 方法

// ToOwned 特性允许我们由 &Borrow 类型得到 Owned 类型，其中 Owned: Borrow<Borrowed>

// 对于任何实现了 Read 特性的类型，其可变的引用类型也实现了 Read 特性。Write 也是如此
struct Person {
    name: String,
}

impl Person {
    // accepts:
    // - String
    fn new1(name: String) -> Person {
        Person { name }
    }

    // accepts:
    // - String
    // - &String
    // - &str
    // - Box<str>
    // - Cow<'_, str>
    // - char
    // since all of the above types can be converted into String
    fn new2<N>(name: N) -> Person
    where
        N: Into<String>,
    {
        Person::new1(name.into())
    }
}

struct Int(i32);

impl FromStr for Int {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Int(s.parse::<i32>()?))
    }
}

// accepts:
//  - &str
//  - &String
fn takes_str(_s: &str) {}

// accepts:
//  - &str
//  - &String
//  - String
fn takes_as_ref_str<S>(s: S)
where
    S: AsRef<str>,
{
    takes_str(s.as_ref());
}

fn _example<I: Iterator<Item = i32>>(mut iter: I) {
    // 任何迭代器的可变引用也是一个迭代器
    let _first3: Vec<i32> = iter.by_ref().take(3).collect();
    for _ in iter {}
}

fn main() {
    let s = map_any(Box::new(2));
    println!("{s:?}");

    let person = Person::new2("123");
    println!("{}", person.name);

    let int: Int = "42".parse().unwrap();
    println!("{}", int.0);

    takes_as_ref_str(String::from("42"));

    let mut bar = Cow::from("Hello World");

    println!("{bar}");

    bar += "24";
    bar.to_mut().push_str("42");

    println!("{bar}");
}
