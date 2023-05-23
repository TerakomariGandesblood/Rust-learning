#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 每个结构体都允许拥有多个 impl 块
// 所有在 impl 块中定义的函数被称为关联函数（associated functions）
impl Rectangle {
    // 不以 self 为第一参数的关联函数（因此不是方法）
    // 经常被用作返回一个结构体新实例的构造函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // 方法（method），第一个参数总是 self（self: &Self 的缩写）
    // 在一个 impl 块中，Self 类型是 impl 块的类型的别名
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 可以和字段同名
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", rect.height);

    let _sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        // 自动引用和解引用（automatic referencing and dereferencing）
        // 当使用 object.something() 调用方法时，
        // Rust 会自动为 object 添加 &，&mut 或 * 以便使 object 与方法签名匹配
        rect.area()
    );

    if rect.width() {
        println!("The rectangle has a nonzero width; it is {}", rect.width);
    }
}
