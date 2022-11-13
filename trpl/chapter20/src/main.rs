use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use chapter20::ThreadPool;

fn main() {
    // 连接到监听端口被称为 “绑定到一个端口”（“binding to a port”）
    // 绑定可能不成功，如端口已被占用，非管理员用户只能监听大于 1024 的端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    // incoming 提供了一系列的流（更准确的说是 TcpStream 类型的流）
    // 流（stream）代表一个客户端和服务端之间打开的连接
    // 连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求 / 响应过程
    for stream in listener.incoming().take(2) {
        // 连接可能不成功，如系统限制同时打开的连接数
        let stream = stream.unwrap();
        // 当 stream 在循环的结尾离开作用域并被丢弃，其连接将被关闭
        thread_pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();
    assert!(n <= 1024);

    // from_utf8_lossy 使用 �，U+FFFD REPLACEMENT CHARACTER，来代替无效序列
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(100));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    let n = stream.write(response.as_bytes()).unwrap();
    assert_eq!(n, response.len());

    // flush 会等待并阻塞程序执行直到所有字节都被写入连接中
    // TcpStream 包含一个内部缓冲区来最小化对底层操作系统的调用
    stream.flush().unwrap();
}
