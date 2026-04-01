use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use chapter21::ThreadPool;

fn main() {
    // 连接到监听端口被称为“绑定到一个端口”（“binding to a port”）
    // 绑定可能不成功，如端口已被占用，非管理员用户只能监听大于 1024 的端口
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    // incoming 提供了一系列的流（更准确的说是 TcpStream 类型的流）
    // 流（stream）代表一个客户端和服务端之间打开的连接
    // 连接（connection）代表客户端连接服务端、服务端生成响应以及服务端关闭连接的全部请求 / 响应过程
    for stream in listener.incoming().take(2) {
        // 连接可能不成功，如系统限制同时打开的连接数
        let stream = stream.unwrap();

        thread_pool.execute(|| handle_connection(stream));
        // 当 stream 在循环的结尾离开作用域并被丢弃，其连接将被关闭
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // 如果数据不是有效的 UTF-8 编码或者读取流遇到问题时，Result 可能是一个错误
    // 连续发送两个换行符来代表一个 HTTP 请求的结束
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(file_name).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
