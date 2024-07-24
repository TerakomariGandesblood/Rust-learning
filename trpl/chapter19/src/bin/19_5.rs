// 宏（Macro）指的是 Rust 中一系列的功能：使用 macro_rules! 的声明（Declarative）宏，和三种过程（Procedural）宏：
// 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码
// 类属性（Attribute-like）宏定义可用于任意项的自定义属性
// 类函数宏看起来像函数不过作用于作为参数传递的 token

// 宏是一种为写其他代码而写代码的方式，即所谓的元编程（metaprogramming）
// 在一个文件里调用宏之前必须定义它，或将其引入作用域，而函数则可以在任何地方定义和调用

// #[macro_export] 如果没有该注解，这个宏不能被引入作用域
// 使用 macro_rules! 来定义声明宏（declarative macros）
#[macro_export]
macro_rules! vec2 {
    // $x:expr，其匹配 Rust 的任意表达式，并将该表达式命名为 $x
    // 逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后
    // * 说明该模式匹配零个或更多个 * 之前的任何模式
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            // 匹配到模式中的 $() 的每一部分，都会在（=>右侧）$()* 里生成 temp_vec.push($x)
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// 过程宏（procedural macros）
// 有三种类型的过程宏（自定义派生（derive），类属性和类函数）

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

// derive 只能用于结构体和枚举
#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    #[allow(clippy::vec_init_then_push)]
    let v = vec2!(1, 2, 3);
    println!("{v:?}");

    Pancakes::hello_macro();
}
