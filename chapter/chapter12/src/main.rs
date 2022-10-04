use chapter12::Config;
use std::{env, process};

fn main() {
    // 第一个值是二进制文件名
    // 使用 unwrap_or_else 可以进行一些自定义的非 panic! 的错误处理
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = chapter12::run(&config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
