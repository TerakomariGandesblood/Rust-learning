// 父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项
// pub 控制的是否跨模块可见
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn _serve_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        // 从父模块开始的相对路径
        super::_serve_order();
        // 从当前模块开始的相对路径
        self::_serve_order();
    }

    fn _serve_order() {}

    // 结构体是公有的，但是其字段还默认是私有的
    // 注意同一模块可以访问私有字段
    pub struct _Breakfast {
        pub toast: String,
        // 注意因为无法设置 seasonal_fruit 的值，所有无法直接创建 _Breakfast 类型的变量
        seasonal_fruit: String,
    }

    // 枚举是公有的则所有成员都是公有的
    pub enum _Appetizer {
        Soup,
        Salad,
    }
}

// 使用 use 将模块引入作用域，按照惯例想引入函数引入其父模块，引入结构体、枚举则直接引入（除非冲突）
// use 语句只适用于其所在的作用域
// use front_of_house::hosting;
// 使用 as 关键字重命名引入作用域的类型
use std::io::Result as IoResult;

// 重导出（re-exporting），使外部代码也可以直接使用 hosting
// pub use 的另一个常见用法是重导出当前 crate 的依赖的定义使其 crate 定义变成你 crate 公有 API
// 的一部分
pub use front_of_house::hosting;

// 嵌套路径
// use std::io::{self, Write};

// 使用 glob 运算符引入所有公有项，常用于测试模块
//use std::collections::*;

fn _func() -> IoResult<()> {
    Ok(())
}

pub fn eat_at_restaurant() {
    // 绝对路径，更倾向于使用绝对路径
    // crate root 在 crate 模块结构的根组成了一个名为 crate 的模块
    // 整个模块树都植根于名为 crate 的隐式模块下
    // 对于外部 crate 的代码，是以 crate 名开头的绝对路径，对于对于当前 crate 的代码，则以字面值
    // crate 开头
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径，从当前模块开始，以 self、super 或当前模块的标识符开头
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
