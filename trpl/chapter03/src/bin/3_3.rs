fn main() {
    println!("{}", func(2));
}

// 必须声明每个参数和返回值（'()'不需要）的类型
// 函数体由一系列的语句和一个可选的结尾表达式构成，函数的返回值等同于函数体最后一个表达式的值
// 语句（statements）是执行一些操作但不返回值的指令；表达式（expressions）计算并产生一个值
// 函数定义是一个语句
// 用大括号创建的一个新的块作用域是一个表达式，代码块的值是其最后一个表达式的值
fn func(a: i32) -> i32 {
    let y = {
        let x = 3;
        x + 1
    };

    let mut _a = 2;
    let mut _b = 2;
    // expected integer, found `()`
    // _a = _b = 3;

    println!("The value of y is: {y}");

    a + 1
}
