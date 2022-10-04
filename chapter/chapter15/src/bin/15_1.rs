// 智能指针（smart pointers）是一类数据结构，它们拥有一些数据并允许你修改它们

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    // *(y.deref())
    assert_eq!(*y, 5);

    // Deref 强制转换（deref coercions）
    // Deref 强制转换只能作用于实现了 Deref trait 的类型。Deref 强制转换将这样一个类型的引用转换为另一个类型的引用
    // 使用任意多次 Deref::deref 调用以获得匹配参数的类型
    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换：
    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
    hello(&MyBox::new(String::from("World")));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
