fn main() {
    // &(3, 5) 会匹配模式 &(x, y)
    let point = (3, 5);
    print_coordinates(&point);
}

// 也可以在闭包参数列表中使用模式
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
