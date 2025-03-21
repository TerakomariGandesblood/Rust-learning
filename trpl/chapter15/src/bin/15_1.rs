// 智能指针（smart
// pointers）是一类数据结构，它们拥有一些数据并允许你修改它们，也拥有额外的元数据和功能

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // 实现了 DerefMut
    let mut p = Box::new(10);
    *p = 20;
    println!("{p}");
}
