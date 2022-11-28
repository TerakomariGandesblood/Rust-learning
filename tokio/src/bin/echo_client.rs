use snmalloc_rs::SnMalloc;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[cfg(not(miri))]
#[global_allocator]
static ALLOC: SnMalloc = SnMalloc;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    // split 内部使用 Arc 和 Mutex
    // let (mut rd, mut wr) = io::split(socket);
    // 使用 Arc，可以移动到其他 task 中
    let (mut rd, mut wr) = socket.into_split();
    // 使用引用，零开销，但是必须和 split 在同一个 task 中
    // let (mut rd, mut wr) = socket.split();

    tokio::spawn(async move {
        wr.write_all(b"hello\n").await?;
        wr.write_all(b"world\n").await?;

        // 这里编译器不能正确推断返回类型
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT {:?}", String::from_utf8(buf[..n].to_vec()).unwrap());
    }

    Ok(())
}
