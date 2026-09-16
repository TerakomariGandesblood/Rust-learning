use std::fmt::Display;
use std::thread;

trait Printable {
    fn stringify(&self) -> String;
}

struct Test(String);

impl Printable for &Test {
    fn stringify(&self) -> String {
        self.0.clone()
    }
}

// 给 trait object 实现关联函数
impl<'a> dyn Printable + 'a {
    fn test(&self) -> String {
        self.stringify()
    }
}

// 通过生命周期省略规则，Rust 自动在第一个函数里推导并添加了一个 'static 约束
fn _dynamic_thread_print(t: Box<dyn Display + Send>) {
    thread::spawn(move || {
        println!("{}", t);
    })
    .join()
    .unwrap();
}

fn _static_thread_print<T: Display + Send + 'static>(t: T) {
    thread::spawn(move || {
        println!("{}", t);
    })
    .join()
    .unwrap();
}

fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}

fn main() {
    let test = Test(String::from("42"));
    let fuck: Box<dyn Printable> = Box::new(&test);
    println!("{}", fuck.stringify());
    println!("{}", fuck.test());

    print_type(&fuck);
}

trait _Trait {}

// T 是 &T 和 &mut T 的超集
// &T 和 &mut T 是不相交的集合
impl<T> _Trait for T {}

// 编译错误
// impl<T> Trait for &T {}

// 编译错误
// impl<T> Trait for &mut T {}

// https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html
// As a trait bound, it means the type does not contain any non-static references
// T: 'static 包含了 &'static T，前者更加灵活，可以接受所有权类型
fn _f<T>(_: T)
where
    T: 'static,
{
}

// As a reference lifetime 'static indicates that the data pointed to by the reference
// lives for the remaining lifetime of the running program
fn _rand_str_generator() -> &'static str {
    let rand_string = "123".to_owned();
    Box::leak(rand_string.into_boxed_str())
}
