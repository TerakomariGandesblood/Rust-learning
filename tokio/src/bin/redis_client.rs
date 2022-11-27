use bytes::Bytes;
use mini_redis::client;
use snmalloc_rs::SnMalloc;
use tokio::sync::{
    mpsc,
    oneshot::{self, Sender},
};

#[global_allocator]
static ALLOC: SnMalloc = SnMalloc;

type Responder<T> = Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    // 32 代表缓冲容量，一旦存储了 32 条消息，再调用 send().await 将挂起
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx.send(Command::Set {
            key: "hello".to_string(),
            val: "world".into(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    })
    .await
    .unwrap();

    tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        tx2.send(Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        })
        .await
        .unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    })
    .await
    .unwrap();

    tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 不存在发送者时，recv 将返回 None
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // 接受者可能不感兴趣，不视为错误
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                }
            }
        }
    })
    .await
    .unwrap();
}
