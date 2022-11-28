use snmalloc_rs::SnMalloc;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[cfg(not(miri))]
#[global_allocator]
static ALLOC: SnMalloc = SnMalloc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // 注意，存储在堆上
            // 如果存储在栈上那么切换任务时会被保存，应该避免
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // 对方已关闭连接，此时立即返回 0
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}
