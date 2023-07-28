fn main() {
    // 类型别名（type alias），用来减少重复
    type Kilometers = i32;
    type _Result<T> = std::result::Result<T, std::io::Error>;

    let y: Kilometers = 5;
    println!("y = {y}");

    // 动态大小类型（dynamically sized types）
    // 有时被称为 DST 或 unsized types，这些类型允许我们处理只有在运行时才知道大小的类型
    // 如 str 是一个 DST，不能创建 str 类型的变量，trait objects 也是 DST
    // &str 不同与常规引用，它保存了两个值：str 的地址和其长度
    // 必须将动态大小类型的值置于某种指针之后
}

// !（never type）：在函数从不返回的时候充当返回值
// never type 可以强转为任何其他类型
// continue 的返回类型是 !
// 不结束的 loop 的类型也是 !
// 从不返回的函数被称为发散函数（diverging functions）
fn _bar() -> ! {
    panic!("panic");
}

// Rust 有一个特定的 trait 来决定一个类型的大小是否在编译时可知：这就是 Sized trait
// 这个 trait 自动为编译器在编译时就知道大小的类型实现
// Rust 隐式的为每一个泛型函数增加了 Sized bound
// 可以使用 ?Sized 放宽这个限制
fn _generic<T: ?Sized>(_: &T) {
    todo!()
}
