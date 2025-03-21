// Rust 并不保证完全地避免内存泄漏（memory leak）

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    _value: i32,
    _parent: RefCell<Weak<Node>>,
    _children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        _value: 3,
        _parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf._parent.borrow().upgrade());

    let branch = Rc::new(Node {
        _value: 5,
        _parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Rc::downgrade 创建弱引用（weak reference），weak_count 递增 1
    *leaf._parent.borrow_mut() = Rc::downgrade(&branch);

    // upgrade 会返回 Option<Rc<T>>，尝试获取对应的 Rc<t>
    println!("leaf parent = {:?}", leaf._parent.borrow().upgrade());
}
