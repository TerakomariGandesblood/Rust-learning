fn main() {
    let mut s = String::from("Hello");
    // 引用，使用值但不获取其所有权
    // &s1 语法创建一个指向值 s 的引用，默认不可变
    // 创建一个引用的行为称为借用（borrowing）
    println!("{}", calculate_length(&s));

    // Temporary lifetime extension
    // NOTE https://doc.rust-lang.org/reference/destructors.html#temporary-lifetime-extension
    let _str = &String::from("string");

    change(&mut s);
    println!("{s}");

    // error: cannot borrow `s` as mutable more than once at a time
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // ok，因为其使用范围没有重叠
    // 编译器在作用域结束之前判断不再使用的引用的能力被称为非词法作用域生命周期（Non-Lexical
    // Lifetimes） NOTE https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#non-lexical-lifetimes
    let r1 = &s;
    println!("{r1}");
    let r2 = &mut s;
    println!("{r2}");

    // 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用
    // 引用必须总是有效的
}

// 使用 & 来表明参数 s 的类型是一个引用
fn calculate_length(s: &str) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" World!")
}

// error: missing lifetime specifier
// this function's return type contains a borrowed value,
// but there is no value for it to be borrowed from
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }
