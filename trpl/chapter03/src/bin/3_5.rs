fn main() {
    let condition = true;
    // if 表达式，条件必须是 bool 类型
    // 两个块返回类型必须相同
    let _number = if condition { 3 } else { 4 };

    loop_label();

    loop_return();

    while_func();

    for_func();
}

fn loop_label() {
    let mut count = 0;

    #[allow(clippy::never_loop)]
    'exit: loop {
        println!("{count}");

        loop {
            count += 1;
            if count == 10 {
                // 跳出多层循环
                break 'exit;
            }
        }
    }
}

fn loop_return() {
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 10 {
            // 从循环返回值
            break count * 2;
        }
    };

    println!("{result}");
}

fn while_func() {
    let mut i = 3;

    while i > 0 {
        println!("{i}");
        i -= 1;
    }
}

fn for_func() {
    let a = [1, 2, 3, 4, 5];

    for ele in a {
        println!("{ele}");
    }

    for i in (1..4).rev() {
        println!("{i}");
    }
}
