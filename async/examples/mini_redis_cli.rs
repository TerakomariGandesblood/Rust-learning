// async 在线程创建、切换的开销显著低于多线程
// async 是基于线程实现，但是它基于线程封装了一个运行时，可以将多个任务映射到少量线程上，然后将线程切换变成了任务切换，后者仅仅是内存中的访问，因此要高效的多
// 应创建一个阻塞的线程去完成相应 CPU 密集任务，因为 CPU 密集的任务很可能会一直霸占着 CPU，此时 tokio 的调度方式决定了该任务会一直被执行
// 使用 spawn_blocking 后，会创建一个单独的 OS 线程，该线程并不会被 tokio 所调度（被 OS 所调度）

use mini_redis::{client, Result};

// #[tokio::main] 的用途
// .await 只能在 async 函数中使用，如果是以前的 fn main，那它内部是无法直接使用 async 函数的
// 异步运行时本身需要初始化
#[tokio::main]
// 通过 async 标记的语法块会被转换成实现了 Future trait 的状态机
// 当 Future 执行并遇到阻塞时，它会让出当前线程的控制权
// 可以把 Future 理解为未来某个时刻会被执行的计划任务
async fn main() -> Result<()> {
    // 在 async 函数中使用 .await 可以等待另一个异步调用的完成，不会阻塞当前的线程
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("从服务器端获取到结果={:?}", result);

    Ok(())
}
