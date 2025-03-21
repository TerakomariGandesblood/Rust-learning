// 测试私有函数
fn _add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// 单元测试（unit tests）
// #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码
// cfg -> configuration
#[cfg(test)]
mod tests {
    use super::*;

    // Rust 中的测试就是带有 test 属性（attribute）注解的函数
    // 当使用 cargo test 命令运行测试时，Rust
    // 会构建一个测试执行程序用来调用被标注的函数，并报告每一个测试是通过还是失败
    #[test]
    // 忽略测试
    #[ignore]
    fn it_works() {
        let result = _add(2, 2);
        // 自定义失败信息，必需参数之后指定的参数都会传递给 format! 宏
        assert_eq!(result, 4, "{result} not equal 4");
        assert!(result == 4);
    }

    #[test]
    // 在 #[test] 之后
    // expected 指定错误信息中包含的文本
    #[should_panic(expected = "a test panic")]
    fn it_will_panic() {
        // 测试失败
        panic!("a test panic");
    }

    // 不能使用 should_panic
    #[test]
    fn result() -> Result<(), String> {
        // 测试成功时不显示
        println!("Hello World!");

        if 2 + 2 == 4 {
            Ok(())
        } else {
            // 测试失败
            Err(String::from("two plus two does not equal four"))
        }
    }
}
