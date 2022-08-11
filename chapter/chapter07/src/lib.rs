// crate 是一个二进制项或者库
// crate root 是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块
// src/main.rs 就是一个与包同名的二进制 crate 的 crate root
// src/lib.rs 就是一个与包同名的库 crate 的 crate root

// 包（package）是提供一系列功能的一个或者多个 crate
// 包中可以包含至多一个库 crate，可以包含任意多个二进制 crate
//（每个 src/bin 下的文件都会被编译成一个独立的二进制 crate），但是必须至少包含一个 crate

// 在另一个与模块同名的文件中加载模块的内容
mod front_of_house;

// 重导出（re-exporting）
// 允许外部代码使用这个新路径
pub use crate::front_of_house::hosting;

// 默认都是私有的
// 子模块中的项可以使用他们父模块中的项
mod back_of_house {
    pub struct _Breakfast {
        // 默认还是私有的
        pub toast: String,
        // 注意因为无法设置 seasonal_fruit 的直，所有无法直接创建 _Breakfast 类型的变量
        seasonal_fruit: String,
    }

    pub enum _Appetizer {
        // 默认公有
        Soup,
        Salad,
    }

    impl _Breakfast {
        pub fn _summer(toast: &str) -> _Breakfast {
            _Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// 嵌套路径
// use std::io::{self, Write};

// 所有公有项引入当前作用域, 常用于测试
// use std::collections::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // 更倾向于使用绝对路径
        // 同一模块, 尽管 front_of_house 是私有的, 也可以访问
        // crate root 在 crate 模块结构的根组成了一个名为 crate 的模块
        // 整个模块树都植根于名为 crate 的隐式模块下
        crate::front_of_house::hosting::_add_to_waitlist();
        super::front_of_house::hosting::_add_to_waitlist();

        // 对于函数, 指定到父模块
        // 对于结构体, 枚举等指定完整路径 (除非冲突)
        use super::front_of_house::hosting;
        hosting::_add_to_waitlist();

        // use std::fmt::Result;
        // use std::io::Result as IoResult;
    }
}
