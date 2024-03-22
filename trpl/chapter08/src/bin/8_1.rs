fn main() {
    let mut v = Vec::new();
    let str = String::from("str");
    // 注意，移动
    v.push(str);
    println!("{v:#?}");

    let v = vec![1, 2, 3];
    println!("{v:#?}");
    // 当 vector 被丢弃时，所有其内容也会被丢弃

    let v = vec![1, 2, 3, 4, 5];
    // 越界则 panic
    // container[index] 是 *container.index(index) 的语法糖
    // See https://doc.rust-lang.org/std/ops/trait.Index.html
    let third = &v[2];
    println!("{third}");

    match v.get(2) {
        Some(num) => println!("{num}"),
        None => println!("There is no third element."),
    }

    // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(1);
    // println!("{}", first);

    // See https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops
    // See https://stackoverflow.com/questions/61907150/is-there-a-difference-between-for-i-in-v-and-for-i-in-v
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i *= 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
