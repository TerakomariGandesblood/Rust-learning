// 如果你在一个 async 代码块中做了大量工作，却没有任何 await 点
// 那么这个 future 就会阻止其他 future 取得进展：starve（饥饿）

use std::thread;
use std::time::Duration;

use trpl::Either;

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            // 控制权交还给运行时
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn _timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    // trpl::select 总是按参数传入的顺序进行轮询
    // 把 future_to_try 作为第一个参数传给 select，好让它即使在 max_time
    // 很短的情况下，也仍然有机会先完成
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
