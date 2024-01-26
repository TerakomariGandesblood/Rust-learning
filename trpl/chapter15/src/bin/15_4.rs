use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 引用计数（reference counting）智能指针
    // Rc<T> 允许在程序的多个部分之间只读地共享数据
    // 注意 Rc<T> 只能用于单线程场景
    // 没有实现 DerefMut
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // 每次调用 Rc::clone，引用计数都会增加
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating a = {}", Rc::strong_count(&a));

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating a = {}", Rc::strong_count(&a));
    }
    println!("count after creating a = {}", Rc::strong_count(&a));
}
