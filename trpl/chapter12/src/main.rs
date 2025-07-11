use std::{env, process};

use chapter12::Config;

// main 函数中的责任应该被限制为：
// 1、使用参数值调用命令行解析逻辑
// 2、设置任何其他的配置
// 3、调用 lib.rs 中的 run 函数
// 4、如果 run 返回错误，则处理这个错误
fn main() {
    // 第一个值是二进制文件名
    // 注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic
    // 如果需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = chapter12::run(&config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
