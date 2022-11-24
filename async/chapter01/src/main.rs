use futures::executor;

struct Song {
    name: String,
}

// 异步函数返回一个 Future
// async 将代码块转换为实现了 Future trait 的状态机
// 被阻塞的 Futures 将让出线程的控制权让其他的 Futures 运行
async fn learn_song() -> Song {
    println!("learn song");
    Song {
        name: "a song".to_string(),
    }
}

async fn sing_song(song: Song) {
    println!("sing {}", song.name);
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    // 使用 .await 等待 learn_song 完成，但是不会阻塞线程
    // 当该 Future 阻塞时，其他 Future 可以接过线程的控制权
    let song = learn_song().await;
    // futures do nothing unless you `.await` or poll them
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // 和 .await 类似，等待多个 Future
    // 一个 Future 阻塞则另一个 Future 接过控制权
    // 如果两个 Future 都阻塞，那么 async_main 阻塞并让出控制权
    futures::join!(f1, f2);
}

fn main() {
    // 阻塞当前线程直到 Future 执行完毕
    executor::block_on(async_main());
}
