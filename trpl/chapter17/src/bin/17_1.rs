// future 是一个现在可能还没有准备好但将在未来某个时刻准备好的值
// 我们称实现了 Future trait 的类型为 future
// 每个 future 会维护自身的进度状态信息以及对“ready”的定义
// 检查一个 future 并查看其值是否已经准备就绪的过程被称为轮询（polling）

// async 关键字可以用于代码块和函数，表明它们可以被中断并恢复
// 在一个 async 块或 async 函数中，可以使用 await 关键字来 await 一个 future（即等待其就绪）
// async 块或 async 函数中每一个等待 future 的地方都可能是一个 async 块
// 或 async 函数中断并随后恢复的点

// Rust 中的 futures 是 惰性（lazy）的：在你使用 await 请求之前它们不会执行任何操作

// 当 Rust 遇到一个 async 关键字标记的代码块时，会将其编译为一个实现了
// Future trait 的唯一的、匿名的 future
// 当 Rust 遇到一个被标记为 async 的函数时，会将其编译成一个函数体是异步代码块的非异步函数
// 异步函数的返回值类型是编译器为异步代码块所创建的匿名数据类型。

use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

// main 函数不能标记为 async
// 因为异步代码需要一个运行时：即一个管理执行异步代码细节的 Rust crate

// Rust 需要记录异步代码块中涉及的状态，这样运行时可以去执行其他工作
// 并在准备好时回来继续推进当前的任务
// Rust 编译器自动创建并管理异步代码的状态机数据结构，由运行时执行这个状态机
fn main() {
    let args = [
        "17_1",
        "https://www.rust-lang.org",
        "https://www.rust-lang.org",
    ];

    trpl::run(async {
        let title_fut_1 = page_title(args[1]);
        let title_fut_2 = page_title(args[2]);

        // 任意一个完成即返回，并取消另一个的执行
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}
