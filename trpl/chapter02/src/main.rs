// Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中
use std::cmp::Ordering;
// 默认情况下，Rust 设定了若干会自动导入到每个程序作用域中的标准库内容
// 这组内容被称为预导入内容（preclude），如果需要的类型不在预导入内容中则需要使用 use
// 显式引入（也可以写全称）
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // rand::rng() 获取随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed
    // 1..=100：范围表达式（range expression）
    let secret_number = rand::rng().random_range(1..=100);
    // NOTE https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html#captured-identifiers-in-format-strings
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // 变量默认不可变
        // String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块
        // new()：关联函数（associated function）
        let mut guess = String::new();

        // stdin 函数返回一个 std::io::Stdin 的实例，这代表终端标准输入句柄的类型
        io::stdin()
            // 读一行，追加到 guess，会保存 \n 或者 \r\n
            // & 表示一个引用，默认不可变
            // 该函数返回值类型为 io::Result（枚举）
            .read_line(&mut guess)
            // 如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect
            // 的信息 如果 io::Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回
            .expect("Failed to read line");

        // 隐藏（shadow），允许我们复用 guess 变量的名字
        // 枚举通常和 match 一同使用
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ 是一个通配符值，用来匹配所有 Err 值
            Err(_) => continue,
        };

        // 也会推断 secret_number 的类型
        match guess.cmp(&secret_number) {
            // 分支（arms）：模式（pattern）+ 代码
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
