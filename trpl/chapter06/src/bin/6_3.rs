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
        return;
    };
    println!("{max}");

    // 当某个值存在时进行一些操作否则返回一个默认
    // 同上，如果模式匹配，它会将匹配到的值绑定到外层作用域。如果模式不匹配，程序流会指向 else
    // 分支，它必须从函数返回
    let Some(max) = config_max else {
        return;
    };
    println!("{max}");
}

fn main() {
    func(Some(3_u8));
    func(None);
}
