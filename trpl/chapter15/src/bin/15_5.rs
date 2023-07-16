// 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据

use std::{cell::RefCell, rc::Rc};

// borrow 方法返回 Ref<T> 类型的智能指针，borrow_mut 方法返回 RefMut<T> 类型的智能指针
// RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针
// 在运行时检查借用规则，违反则 panic
trait Messenger {
    fn send(&self, msg: &str);
}

struct Email {}

impl Email {
    fn new() -> Email {
        Email {}
    }
}

impl Messenger for Email {
    fn send(&self, msg: &str) {
        println!("sent email: {msg}");
    }
}

struct LimitTracker<'a, T>
where
    T: Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let email = Email::new();
    let mut limit_tracker = LimitTracker::new(&email, 100);
    limit_tracker.set_value(80);
    email.send("message");

    let value = Rc::new(RefCell::new(42));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(2)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
