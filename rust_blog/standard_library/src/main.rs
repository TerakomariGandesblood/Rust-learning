// 可自动实现的特性 Auto Traits
// 若给定类型的成员都实现了该特性，那么该类型就隐式地自动实现该特性
// 如：unsafe auto trait Send {}

// 所有的特性都具有隐式的 ?Sized 约束
// trait Trait: ?Sized {}

use std::any::Any;

fn map_any(mut any: Box<dyn Any>) -> Box<dyn Any> {
    if let Some(num) = any.downcast_mut::<i32>() {
        *num += 1;
    } else if let Some(string) = any.downcast_mut::<String>() {
        *string += "!";
    }

    any
}

fn main() {
    let s = map_any(Box::new(2));
    println!("{s:?}");
}
