use std::pin::pin;
use std::time::Duration;

// Ext 是“extension”：在 Rust 社区中这是用另一个 trait 扩展 trait 的常见模式
// Stream trait 定义了一个底层接口用于有效地结合 Iterator 与 Future trait
// StreamExt trait 在 Stream 之上提供了一组高层 API，其中包括了 next 和其它类似于 Iterator
// trait 提供的工具方法
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let values = 1..10;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });

    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            // 限流（Throttling）是一种限制函数被调用速率的方式，或者在本例中是限制流被轮询的频率
            // 不会有大量未处理的间隔消息来选择性地丢弃，我们最开始就从未产生这些间隔消息
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                // Ok 变体表明消息及时到达；Err 变体表明任何消息到达前就触发超时了
                // 超时最终并不会阻止消息到达
                // 因为我们的信道是无限的（unbounded）：它可以存储内存所允许的所有消息
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
