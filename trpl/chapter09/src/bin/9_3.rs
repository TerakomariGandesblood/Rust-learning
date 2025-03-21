// 示例、代码原型和测试都非常适合 panic
// 在确定不会出现 Err 时，可以用 unwrap 或者 expect

// 在当有可能会导致有害状态的情况下建议使用 panic!
// 有害状态是指当一些假设、保证、协议或不可变性被打破的状态，例如无效的值、
// 自相矛盾的值或者被传递了不存在的值，还有：1、有害状态是非预期的行为，与偶尔会发生的行为相对，
// 比如用户输入了错误格式的数据。2、在此之后代码的运行依赖于不处于这种有害状态，
// 而不是在每一步都检查是否有问题。3、没有可行的手段来将有害状态信息编码进所使用的类型中的情况。

// 当代码对值进行操作时，应该首先验证值是有效的，并在其无效时 panic!
// 这主要是出于安全的原因：尝试操作无效数据会暴露代码漏洞

// 创建自定义类型进行有效性验证
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let guess = Guess::new(100);
    println!("{}", guess.value());
}
