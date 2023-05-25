fn main() {
    // slice 允许引用集合中一段连续的元素序列，见
    // NOTE https://doc.rust-lang.org/std/primitive.slice.html
    // NOTE https://doc.rust-lang.org/std/primitive.str.html

    let /*mut*/ s = String::from("Hello World");
    let word = first_word(&s);
    // error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();
    println!("{word}");

    // 0 可以省略
    let _hello = &s[0..5];
    // 11 可以省略
    let _world = &s[6..11];
    // 如果尝试从一个多字节字符的中间位置创建字符串 slice，会 panic

    let str = "Hello World!";
    // str[0..2] 的类型是 str
    first_word(&str[0..2]);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// function arguments must have a statically known size
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
