// Rust 将错误组合成两个主要类别
// 可恢复错误（recoverable），希望向用户报告错误并重试操作，如未找到文件，用 Result 处理
// 不可恢复错误（unrecoverable），通常是 bug，如尝试访问超过数组结尾的位置，用 panic! 处理

fn main() {
    // 默认展开（unwinding），回溯栈并清理它遇到的每一个函数的数据
    // 也可以选择终止（abort），直接退出
    // 可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace
    panic!("Error");
}
