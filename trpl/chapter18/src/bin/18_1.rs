fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = vec![1, 2, 3];

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = ['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // 下面的也是模式
    let _x = 5;
    let (_x, _y, _) = (1, 2, 3);
    let (_x, ..) = (1, 2, 3);
    let &_x = &_x;

    // &(3, 5) 会匹配模式 &(x, y)
    let point = (3, 5);
    print_coordinates(&point);
}

// 也可以在闭包参数列表中使用模式
// 注意如果没有实现 Copy，则会试图移动
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
