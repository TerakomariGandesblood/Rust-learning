// Rust 标准库使用 1:1 线程实现，这代表程序的每一个语言级线程使用一个系统线程
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个新线程
    // 执行顺序依赖操作系统如何调度线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            // 强制线程停止执行一小段时间
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // JoinHandle 是一个拥有所有权的值
    // 当对其调用 join 方法时，它会阻塞（Blocking）当前线程直到 handle 所代表的线程结束
    // 阻塞线程意味着阻止该线程执行工作或退出
    handle.join().unwrap();

    {
        let v = vec![1, 2, 3];
        // Rust 不知道这个新建线程会执行多久，所以无法知晓 v 的引用是否一直有效
        // 通过增加 move 关键字，强制闭包获取其使用的值的所有权
        // T: 'static 是指 T 可以被安全地无期限地持有，甚至可以直到程序结束
        // 见 https://github.com/pretzelhammer/rust-blog/blob/master/posts/translations/zh-hans/common-rust-lifetime-misconceptions.md#2-如果-t-static-那么-t-直到程序结束为止都一定是有效的
        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        handle.join().unwrap();
    }

    {
        let mut v = vec![1, 2, 3];

        // 可以借用 non-'static 数据，在 scope 返回前，所有未手动 join 的线程将自动 join
        thread::scope(|s| {
            s.spawn(|| dbg!(&v));
        });

        v.push(4);
        dbg!(&v);
    }
}
