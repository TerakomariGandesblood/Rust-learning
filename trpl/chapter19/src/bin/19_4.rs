fn add_one(x: i32) -> i32 {
    x + 1
}

// 函数指针（function pointer）
// 函数指针实现了所有三个闭包 trait（Fn、FnMut 和
// FnOnce），所以总是可以在调用期望闭包的函数时传递函数指针作为参数 倾向于编写使用泛型和闭包 trait
// 的函数，这样它就能接受函数或闭包作为参数
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 返回闭包
fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn _return_fn() -> fn(i32) -> i32 {
    add_one
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    let list_of_numbers = [1, 2, 3];
    let _list_of_string: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let _list_of_string: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // 这些项使用 () 作为初始化语法，这看起来就像函数调用，
    // 同时它们确实被实现为返回由参数构造的实例的函数
    enum Status {
        Value(u32),
    }
    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
