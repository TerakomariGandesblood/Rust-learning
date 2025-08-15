use std::time::Duration;

fn main() {
    {
        // 外面的代码会阻塞到 run 函数返回
        trpl::run(async {
            let handle = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            // 等待执行结束
            handle.await.unwrap();

            let fut1 = async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let fut2 = async {
                for i in 1..5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            // 等待二者执行结束
            // trpl::join 函数是公平的（fair），这意味着它以相同的频率检查每一个
            // future，使它们交替执行
            trpl::join(fut1, fut2).await;
        });
    }

    {
        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                // recv 不会阻塞
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            trpl::join(tx_fut, rx_fut).await;
        });
    }
}
