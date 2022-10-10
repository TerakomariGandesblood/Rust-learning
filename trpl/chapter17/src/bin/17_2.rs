trait Draw {
    fn draw(&self);
}

struct Screen {
    // trait 对象（trait object），代表了实现了 Draw trait 的类型，允许对通用行为进行抽象
    // 只有对象安全（object-safe）的 trait 可以实现为 trait 对象
    // 如果一个 trait 中定义的所有方法都符合以下规则，则该 trait 是对象安全的
    // 返回值不是 Self；没有泛型类型的参数
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

    // Rust 在运行时决定使用哪个方法实现
    screen.run();
}
