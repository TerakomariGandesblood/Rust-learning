fn main() {
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            // 注意隐藏了外部的 y
            Some(y) => println!("Matched, y = {y:?}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y:?}");
    }

    {
        let x = 1;

        match x {
            // 多个模式
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = 5;

        match x {
            // 匹配 [1,5]
            // 只允许数字和 char
            // 目前仅支持闭区间
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    {
        let p = Point { x: 0, y: 7 };

        // let Point { x: x, y: y } = p;
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // 简写：只需列出结构体字段的名称，则模式创建的变量会有相同的名称
        let Point { x, y } = p;
        assert_eq!(0, x);
        assert_eq!(7, y);
    }

    {
        let p = Point { x: 0, y: 7 };

        match p {
            // 使用字面值作为结构体模式的一部分进行解构，而不是为所有的字段创建变量
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => println!("On neither axis: ({x}, {y})"),
        }
    }

    {
        let msg = Message::_ChangeColor(0, 160, 255);

        match msg {
            Message::_Quit => {
                println!("The Quit variant has no data to destructure.")
            }
            Message::_Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::_Write(text) => println!("Text message: {text}"),
            Message::_ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
    }

    let s = Some(String::from("Hello!"));

    // 注意移动，如果使用 _，则不会发生移动
    if let Some(_s) = s {
        println!("found a string");
    }

    {
        struct Point {
            _x: i32,
            _y: i32,
            _z: i32,
        }

        let origin = Point {
            _x: 0,
            _y: 0,
            _z: 0,
        };

        // 使用 .. 忽略剩余值
        // let Point { _x, _y: _, _z: _ } = origin;
        let Point { _x, .. } = origin;
        println!("{_x}");
    }

    {
        let numbers = (2, 4, 8, 16, 32);
        // (.., second, ..) 则报错，.. 必须是无歧义的
        let (_first, .., _last) = numbers;
    }

    {
        let num = Some(4);

        match num {
            // 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if
            // 条件，它也必须被满足才能选择此分支
            // 当涉及匹配守卫表达式时编译器不会尝试检查穷尽性
            Some(x) if x < 5 => println!("less than five: {x}"),
            Some(x) => println!("{x}"),
            None => (),
        }
    }

    {
        let x = 4;
        let y = false;

        match x {
            // 这时匹配守卫将会作用于所有的模式
            #[allow(clippy::manual_range_patterns)]
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                // at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式
                // 同样只能是闭区间
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }

        if let Some(x @ 1..=5) = Some(30) {
            println!("{x}");
        } else {
            println!("no");
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

// 在名字前以一个下划线开头来忽略未使用的变量
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
}

// 使用 _ 模式忽略整个值
fn _foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
