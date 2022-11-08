// 不允许只将某个字段标记为可变的
struct User {
    _active: bool,
    // 使用自身没有所有权的类型需要加生命周期
    _username: String,
    _email: String,
    _sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let _user2 = User {
        _email: String::from("another@example.com"),
        // 结构体更新语法（struct update syntax）
        // 其他值来自 user1
        // 注意这样做类似赋值，没有实现 Copy trait 的字段将会无效（注意直接将字段赋值也会发生移动）
        ..user1
    };

    // 元组结构体（tuple structs）
    // 两者类型不同，其他和元组一致
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // 类单元结构体（unit-like structs）
    // 和 '()' 类似
    // 常常在想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用
    struct AlwaysEqual;
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        // 参数名与字段名完全相同时可以省略字段名——字段初始化简写语（field init shorthand）
        _email: email,
        _username: username,
        _active: true,
        _sign_in_count: 1,
    }
}
