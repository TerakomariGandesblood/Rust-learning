use std::future::Future;

use futures::executor;

async fn foo() -> u8 {
    5
}

#[allow(clippy::manual_async_fn)]
fn bar() -> impl Future<Output = u8> {
    // async blocks，和函数一样都返回 impl Future<Output = ..>
    async {
        // .await 会尝试完成 foo 的运行，但是当其阻塞时，则会让出控制权
        // 当可以进一步运行时，它将会被 executor 挑出并恢复执行
        let x = foo().await;
        x + 5
    }
}

async fn _foo(x: &u8) -> u8 {
    *x
}

// 接受一个引用或者非 'static 参数的 async 函数，其返回的 Future 具有相同的生命周期
#[allow(clippy::manual_async_fn)]
#[allow(clippy::needless_lifetimes)]
fn _foo_expanded<'a>(x: &'a u8) -> impl Future<Output = &u8> + 'a {
    async move { x }
}

// 当使用一个多线程的 Future executor，Future 可能在线程间移动
// 任何在 async body 中使用的变量都需要实现 Send trait
// 任何 .await 都可能切换到新的线程
// 应该使用 futures::lock::Mutex 代替 std::sync::Mutex，因为后者可能造成线程池锁住

fn main() {
    let futures = bar();
    let x = executor::block_on(futures);
    println!("{x}");
}
