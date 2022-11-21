// 不并行运行测试
// cargo test -- --test-threads=1
// 显示通过的测试中打印的内容
// cargo test -- --show-output
// 运行指定的测试（任何名称匹配这个名称的测试会被运行）
// cargo test test_name
// 只运行忽略的测试
// cargo test -- --ignored
// 运行全部测试
// cargo test -- --include-ignored

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
    #[test]
    // 忽略测试
    #[ignore]
    fn it_works() {
        let result = _add(2, 2);
        // 自定义失败信息
        assert_eq!(result, 4, "{} not equal 4", result);
        assert!(result == 4);
    }

    #[test]
    // 在 #[test] 之后
    #[should_panic(expected = "a test panic")]
    fn it_will_panic() {
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
