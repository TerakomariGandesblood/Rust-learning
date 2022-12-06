// TODO The Future implementation

use std::{io, time::Duration};

use tokio::{
    net::TcpListener,
    sync::{mpsc, oneshot},
    time,
};

async fn some_operation() -> String {
    time::sleep(Duration::from_secs(1)).await;
    "some operation".to_string()
}

#[tokio::main]
async fn main() {
    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        tokio::select! {
            val = some_operation() => {
                println!("tx1.send");
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                println!("tx1.closed");
            }
        }
    });

    tokio::spawn(async {
        println!("tx2.send");
        let _ = tx2.send("two");
    });

    // 在一个 task 中执行分支，
    // 随机选择一个分支尝试执行，如果该分支尚未准备好，则换一个分支尝试执行
    // 任何一个分支完成将执行关联的块，没完成的分支将被 drop
    // 最多支持 64 个分支
    // <pattern> = <async expression> (, if <precondition>)? => <handler>,
    // 首先检查 <precondition>
    // 如果匹配失败，则剩余的 <async expression> 继续执行
    // <async expression> 中可以借用数据，视为同一作用域
    // 在 <handler> 中借用，视为不同作用域
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
}

async fn _func() -> io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        tx.send(()).unwrap();
    });

    let listener = TcpListener::bind("localhost:3465").await?;

    tokio::select! {
        res = async {
            // 在这里使用 ?，<async expression> 将变为 Result
            let (_, _) = listener.accept().await?;
            Ok::<_, io::Error>(())
        } => {
            // 终结 main，并返回 Result
            res?;
        }
        _ = rx => {
            println!("terminating accept loop");
        }
    }

    Ok(())
}

async fn _func2() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send(1).await.unwrap();
        tx2.send(2).await.unwrap();
    });

    tokio::select! {
        Some(v) = rx1.recv() => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = rx2.recv() => {
            println!("Got {:?} from rx2", v);
        }
        // 其他都无法匹配时 else 分支将执行
        else => {
            println!("Both channels closed");
        }
    }
}

async fn _func3() {
    let (tx1, mut rx1) = mpsc::channel(128);
    let (tx2, mut rx2) = mpsc::channel(128);
    let (tx3, mut rx3) = mpsc::channel(128);

    tokio::spawn(async move {
        tx1.send(1).await.unwrap();
        tx2.send(2).await.unwrap();
        tx3.send(3).await.unwrap();
    });
    // select! 常在循环中使用
    loop {
        let msg = tokio::select! {
            Some(msg) = rx1.recv() => msg,
            Some(msg) = rx2.recv() => msg,
            Some(msg) = rx3.recv() => msg,
            else => { break }
        };

        println!("Got {:?}", msg);
    }

    println!("All channels have been closed.");
}

async fn _action(input: Option<i32>) -> Option<String> {
    Some(input?.to_string())
}

async fn _func4() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    let mut done = false;
    let operation = _action(None);
    // 如果要在一个引用上使用 .await，那么引用必须 pinned 或实现了 Unpin
    tokio::pin!(operation);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        let _ = tx.send(3).await;
        let _ = tx.send(2).await;
    });

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;

                if let Some(v) = res {
                    println!("GOT = {}", v);
                    return;
                }
            }
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    operation.set(_action(Some(v)));
                    done = false;
                }
            }
        }
    }
}
