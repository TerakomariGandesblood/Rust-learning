// 智能指针（smart pointers）是一类数据结构，它们拥有一些数据并允许你修改它们

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
