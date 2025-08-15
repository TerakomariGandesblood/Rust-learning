use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 使用 mpsc::channel
    // 函数创建一个新的信道，返回一个元组：第一个元素是发送端，而第二个元素是接收端
    // mpsc 是多个生产者，单个消费者（multiple producer, single consumer）的缩写
    // 由于历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写
    let (tx, rx) = mpsc::channel();
    // 通过克隆发送者来创建多个生产者
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // 如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误
            // 获取 val 的所有权，因为将值发送到另一个线程后，
            // 那个线程可能会在我们再次使用它之前就将其修改或者丢弃
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    // recv 会阻塞主线程执行直到从信道中接收一个值
    // 当信道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了
    // try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，
    // 而 Err 值代表此时没有任何消息
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // 当信道被关闭时，迭代器也将结束
    for received in rx {
        println!("Got: {received}");
    }
}
