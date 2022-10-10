use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        // 生成一个新的 Tokio 任务
        // 任务是调度器管理的执行单元。spawn生成的任务会首先提交给调度器，然后由它负责调度执行
        // 当使用 Tokio 创建一个任务时，该任务类型的生命周期必须是 'static。意味着，在任务中不能使用外部数据的引用
        tokio::spawn(async move {
            process(stream).await;
        });
    }
}

async fn process(stream: TcpStream) {
    let mut connection = Connection::new(stream);
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
