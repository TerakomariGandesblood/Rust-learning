use std::sync::{Arc, Mutex};
use std::thread;

// 互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据
// 互斥器为通过锁系统保护（guarding）其数据，
// 线程首先需要通过获取互斥器的锁（lock）来表明其希望访问数据
fn main() {
    // 原子引用计数（atomically reference counted），可以安全的在线程间共享
    // Mutex<T> 提供了内部可变性，有造成死锁（deadlock）的风险
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // 使用 lock 方法获取锁，以访问互斥器中的数据。这个调用会阻塞当前线程，
            // 直到我们拥有锁为止
            // 如果另一个线程拥有锁，并且那个线程 panic 了，则 lock
            // 调用会失败，在这种情况下，没人能够再获取锁
            // 一旦获取了锁，就可以将返回值视为一个其内部数据的可变引用了
            // lock 调用 返回 一个叫做 MutexGuard 的智能指针，提供了一个 Drop 实现当 MutexGuard
            // 离开作用域时自动释放锁
            *counter.lock().unwrap() += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());
}
