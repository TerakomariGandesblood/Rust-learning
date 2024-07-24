// 增加属性来派生 Debug trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // :? -> Debug 输出格式
    // :#? -> 更易读的输出
    println!("{rect:?}");
    // 另一种方式，但是打印到 stderr
    // dbg! 宏接收并返回一个表达式的所有权
    dbg!(&rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
