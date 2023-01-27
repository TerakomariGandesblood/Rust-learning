fn func(config_max: Option<u8>) {
    // if let 语法获取通过等号分隔的一个模式和一个表达式
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        // 相当于 _
        println!("No maximum");
    }

    let max = if let Some(max) = config_max {
        max
    } else {
        panic!("No maximum");
    };
    println!("{max}");

    // 同上
    // NOTE https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html#let-else-statements
    let Some(max) = config_max else {
        // 只能是 panic!、return、break 等
        panic!("No maximum");
    };
    println!("{max}");
}

fn main() {
    func(Some(3_u8));
    func(None);
}
