use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // 可以选择注明类型，不注明则需要调用
        // 如果闭包体只有一行则大括号可以省略
        // 每个闭包的类型都是不同的
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {user_pref1:?} gets {giveaway1:?}");

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {user_pref2:?} gets {giveaway2:?}");

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // 捕获名为 list 的 vector 的不可变引用
        let only_borrows = || println!("From closure: {list:?}");

        println!("Before calling closure: {list:?}");
        only_borrows();
        println!("After calling closure: {list:?}");
    }

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7);

        // cannot borrow `list` as immutable because it is also borrowed as mutable
        // println!("{list:?}");

        borrows_mutably();
        println!("After calling closure: {list:?}");
    }

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // 强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move
        // 关键字，如将数据移动到新线程
        let move_closure = move || println!("From thread: {list:?}");
        thread::spawn(move_closure).join().unwrap();
    }

    // 闭包自动、渐进地实现一个、两个或三个 Fn trait（函数都实现了这三个 trait）
    // 如果我们要做的事情不需要从环境中捕获值，则可以在需要某种实现了 Fn trait
    // 的东西时使用函数而不是闭包 可以在 Option<Vec<T>> 的值上调用 unwrap_or_else(Vec::new)
    // 以便在值为 None 时获取一个新的空的 vector

    {
        // 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce
        // 一个会将捕获的值移出闭包体的闭包只实现 FnOnce trait，这是因为它只能被调用一次
        let list = vec![1, 2, 3];
        let once_closure = || list;
        println!("{:?}", once_closure());
    }

    {
        // FnMut 适用于不会将捕获的值移出闭包体的闭包，但它可能会修改被捕获的值，可以调用多次
        let mut list = vec![1, 2, 3];
        let mut mut_closure = || list.push(42);
        mut_closure();
        println!("{list:?}");
    }

    {
        // Fn 适用于既不将被捕获的值移出闭包体也不修改被捕获的值的闭包，
        // 当然也包括不从环境中捕获值的闭包 这类闭包可以被调用多次而不改变它们的环境，
        // 这在会多次并发调用闭包的场景中十分重要
        let list = [1, 2, 3];
        let closure = || list.len();
        println!("{}", closure());
    }

    {
        let mut list = [
            Rectangle {
                width: 10,
                _height: 1,
            },
            Rectangle {
                width: 3,
                _height: 5,
            },
            Rectangle {
                width: 7,
                _height: 12,
            },
        ];

        // 会调用多次
        list.sort_by_key(|r| r.width);
        println!("{list:?}");
    }
}
