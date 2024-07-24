enum IpAddr {
    V4(u8, u8, u8, u8),
    // 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数
    // 也就是说，IpAddr::V6() 是一个获取 String 参数并返回 IpAddr 类型实例的函数调用
    V6(String),
}

enum _Message {
    // 和结构体定义语法类似
    Quit,
    // 包含命名字段
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 可以在枚举上定义方法
impl _Message {
    fn _call(&self) {}
}

fn main() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    // Option<T> 包含在 prelude 中，它的成员也是如此
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;
}
