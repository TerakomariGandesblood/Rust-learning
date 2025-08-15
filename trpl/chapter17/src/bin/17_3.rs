// Rust 只会 在一个 await point 暂停异步代码块并将控制权交还给运行时
// await points 之间的一切都是同步的

use std::thread;
use std::time::Duration;

use trpl::Either;

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    {
        use std::pin::{Pin, pin};

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = pin!(async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            let rx_fut = pin!(async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            });

            let tx_fut = pin!(async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            });

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

            // impl<F, A> Future for Box<F, A>
            // where
            //     F: Future + Unpin + ?Sized,
            //     A: Allocator,

            // 处理动态数量的 future，只要它们都有相同的类型
            trpl::join_all(futures).await;

            let a = async { 1u32 };
            let b = async { "Hello!" };
            let c = async { true };

            // 处理固定数量的 future，哪怕它们有着不同的类型
            let (a_result, b_result, c_result) = trpl::join!(a, b, c);
            println!("{a_result}, {b_result}, {c_result}");

            let slow = async {
                println!("'slow' started.");
                trpl::sleep(Duration::from_millis(100)).await;
                println!("'slow' finished.");
            };

            let fast = async {
                println!("'fast' started.");
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'fast' finished.");
            };

            // race 是不公平的，它总是以传递的参数的顺序来运行传递的 futures
            trpl::race(fast, slow).await;
        });
    }

    {
        trpl::run(async {
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                // 交还控制权给运行时
                trpl::yield_now().await;
                slow("a", 10);
                slow("a", 20);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                slow("b", 10);
                slow("b", 15);
                slow("b", 350);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
        });
    }

    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
