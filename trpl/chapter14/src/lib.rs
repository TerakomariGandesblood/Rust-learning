//! # My Crate
//! 该类型的注释这通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件为 crate
//! 或模块整体提供文档

// 文档注释（documentation comments）
// 支持 Markdown 注解来格式化文本
// cargo doc --open 会构建当前 crate 文档（包括依赖），并打开浏览器
// 常用的标题有：
// Examples：示例
// Panics：描述函数可能会 panic! 的场景
// Errors：该函数返回 Result，描述可能会出现何种错误以及什么情况会造成这些错误
// Safety：这个函数使用 unsafe 代码，描述期望函数调用者支持的确保 unsafe
// 块中代码正常工作的不变条件（invariants）

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chapter14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(3), 4);
    }
}

// cargo publish 发布
// 发布是永久性的（permanent），对应版本不可能被覆盖，其代码也不可能被删除
// cargo yank 撤回版本，撤回意味着所有带有 Cargo.lock 的项目的依赖不会被破坏，同时任何新生成的
// Cargo.lock 将不能使用被撤回的版本
// 如果 $PATH 中有类似 cargo-something 的二进制文件，就可以通过
// cargo something 来像 Cargo 子命令一样运行它 cargo --list 列出子命令
