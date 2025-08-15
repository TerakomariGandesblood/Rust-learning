// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）
// 能匹配任何传递的可能值的模式被称为是不可反驳的（irrefutable），如 let x = 5;
// 对某些可能的值进行匹配会失败的模式被称为是可反驳的（refutable），如 if let Some(x) = a_value
// 函数参数、let 语句和 for 循环只能接受不可反驳的模式
// if let 和 while let 表达式被限制为只能接受可反驳的模式

fn main() {
    let option = Some(42);
    let Some(x) = option else { todo!() };
    println!("{x}");
}
