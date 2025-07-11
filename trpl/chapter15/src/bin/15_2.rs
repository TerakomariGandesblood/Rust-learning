use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    // Rust 事实上在底层运行了如下代码：*(y.deref())，只会发生一次，不会递归替换
    assert_eq!(*y, 5);

    // Deref 强制转换（deref coercions）
    // Deref 强制转换只能作用于实现了 Deref trait 的类型。
    // Deref 强制转换将这样一个类型的引用转换为另一个类型的引用
    // 使用任意多次 Deref::deref 调用以获得匹配参数的类型
    // Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换
    // 当 T: Deref<Target=U> 时从 &T 到 &U。
    // 当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U。
    // 当 T: Deref<Target=U> 时从 &mut T 到 &U。
    hello(&MyBox::new(String::from("World")));
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
