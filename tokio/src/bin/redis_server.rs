use std::sync::Arc;

use snmalloc_rs::SnMalloc;
// Bytes 相比与 Vec<u8>，调用 clone 时不会拷贝底层数据，类似于 Arc<Vec<u8>>
use bytes::Bytes;
// DashMap 是一个线程安全的 HashMap
use dashmap::DashMap;
use mini_redis::Command::{self, Get, Set};
use mini_redis::Frame;
// std::sync::Mutex 更快的替代品
// use parking_lot::Mutex;
use tokio::net::{TcpListener, TcpStream};

// 注意这里使用了 std::sync::Mutex 而不是 tokio::sync::Mutex
// tokio::sync::Mutex 应该在可能在线程间移动时使用，开销更高
// 注意当获取锁时阻塞，不仅会影响当前的 task，还会影响该线程上被调度的所有 task，当 task 很多时，存在竞争问题
// type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type Db = Arc<DashMap<String, Bytes>>;

#[cfg(not(miri))]
#[global_allocator]
static ALLOC: SnMalloc = SnMalloc;

#[path = "../frame.rs"]
mod conn;

use conn::Connection;

// async fn 将在编译时翻译为异步执行的程序，需要由 runtime 执行（默认使用多线程调度器），其不会自动自动，需要由 main 初始化
// main 不能是 async 的
#[tokio::main]
async fn main() {
    // async fn 返回具有实现了 Future trait 的类型的值，称为 future
    // .await 将控制权让出给调度器
    // 注意，Rust 的异步操作是 lazy 的，没有 .await 将什么都不做
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // 这里 db 是一个 handle，意味着其可以提供对某些共享状态的访问
    let db = Arc::new(DashMap::new());

    loop {
        let (socket, _) = listener.accept().await.unwrap();

        let db = Arc::clone(&db);

        // Tokio task 是一个异步 green thread，由传递一个 async block 给 tokio::spawn 创建
        // tokio::spawn 返回 JoinHandle，使用 .await 可以获得 async block 的返回值
        // 注意调用 spawn 后 task 立刻开始运行
        // Tokio task 是由调度器管理的执行单位，可能在与创建它不同的线程上执行，也可能在线程间传递
        // task 十分轻量，只需要一次内存分配和 64 字节内存
        // green thread 是一种由运行环境或虚拟机调度，而不是由本地底层操作系统调度的线程
        // 传递给 spawn 的 async block 类型的 lifetime 必须是 'static，也就是不含对任何外部数据的引用，因为无法确定 task 会运行多久
        // 也必须实现 Send trait，因为当使用 .await 挂起当前任务时，可能在线程间传递 task
        // 当使用 .await 时要保存的所有数据是 Send 时，task 是 Send 的
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // Connection 可以读写 redis frames
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
