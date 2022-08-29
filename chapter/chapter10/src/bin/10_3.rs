fn main() {
    let str1 = String::from("abc");
    let str2 = "ab";

    let result = longest(&str1, str2);
    println!("{}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        _part: first_sentence,
    };

    // 静态生命周期
    let _str: &'static str = "string";
}

// 该函数返回的引用的生命周期至少和参数的生命周期的较小者一致
// 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
// 一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// 这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久
struct ImportantExcerpt<'a> {
    _part: &'a str,
}

// 生命周期省略规则（lifetime elision rules）
// 这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期
// 1、每一个是引用的参数都有它自己的生命周期参数
// 2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
// 3、如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，那么所有输出生命周期参数被赋予 self 的生命周期
// 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误

impl<'a> ImportantExcerpt<'a> {
    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("{}", announcement);
        self._part
    }
}
