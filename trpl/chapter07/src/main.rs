// crate 是 Rust 在编译时最小的代码单位

// crate 是一个二进制项或者库
// crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块
// src/main.rs 就是一个与包同名的二进制 crate 的 crate root
// src/lib.rs 就是一个与包同名的库 crate 的 crate root

// 包（package）是提供一系列功能的一个或者多个 crate
// 包中可以包含至多一个库 crate，可以包含任意多个二进制 crate
//（每个 src/bin 下的文件都会被编译成一个独立的二进制 crate），但是必须至少包含一个 crate

use crate::garden::vegetables::Asparagus;

// 在 src/garden.rs 或 src/garden/mod.rs 寻找模块代码
mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
    chapter07::eat_at_restaurant();
}
