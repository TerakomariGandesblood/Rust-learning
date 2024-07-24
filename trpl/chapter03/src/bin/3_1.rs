// 常量（constants）必须注明类型，只能被设置为常量表达式
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// NOTE https://blog.rust-lang.org/2017/04/27/Rust-1.17.html#whats-in-1170-stable
// const 和 static 的引用隐含 'static
const NAME: &str = "Ferris";
static NAME2: &str = "Ferris";

fn main() {
    println!("{THREE_HOURS_IN_SECONDS}");
    println!("{NAME}");
    println!("{NAME2}");
}
