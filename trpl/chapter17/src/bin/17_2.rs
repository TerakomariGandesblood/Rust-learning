use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        // join 会生成一个新的 future
        // trpl::join 是 fair 的，也就是它会以同样的频率检查每一个 future，在它们之间交替进行
        trpl::join(fut1, fut2).await;

        let (tx, mut rx) = trpl::channel();

        // 将所有权移入 async 代码块
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
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        // trpl::join!() 可以 await 多个 future
        trpl::join(tx_fut, rx_fut).await;
    });
}
