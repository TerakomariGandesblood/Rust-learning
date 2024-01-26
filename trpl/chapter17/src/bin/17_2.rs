trait Draw {
    fn draw(&self);
}

struct Screen {
    // trait 对象（trait object），代表了实现了 trait 的类型，允许对通用行为进行抽象
    // trait 对象指向一个实现了我们指定 trait 的类型的实例，以及一个用于在运行时查找该类型的 trait 方法的表
    // 只有对象安全（object-safe）的 trait 可以实现为 trait 对象
    // 见 https://doc.rust-lang.org/reference/items/traits.html#object-safety
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button");
    }
}

fn main() {
    let screen = Screen {
        components: vec![Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        })],
    };

    // 单态化产生的代码在执行静态分发（static dispatch）
    // 静态分发发生于编译器在编译时就知晓调用了什么方法的时候
    // 这与动态分发（dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法
    // 当使用 trait 对象时，Rust 必须使用动态分发
    // Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法
    // 动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化
    screen.run();
}
