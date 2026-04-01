// https://night-cruise.github.io/async-rust/async-await/Pin.html

// 对 self 进行类型注解
// 它告诉 Rust：要调用这个方法，self 必须是什么类型。
// 它不能随便写成任意类型。它必须是方法所实现类型本身、该类型的引用或智能指针，
// 或者是一个包裹了该类型引用的 Pin。

// future 中存在自引用结构
// Pin 是一个让编译器能够对指针使用方式施加约束的工具
// 只有实现了 Unpin 的类型，才能从 Pin<P<T>> 里拿到 &mut T（T 是指针 P 指向的类型）
// Unpin 告诉编译器，某个类型不需要维护“这个值是否可以安全移动”方面的额外保证
// Rust 会主动为 future 实现 !Unpin

use std::pin::{self, Pin};

fn main() {
    trpl::block_on(async {
        let tx1_fut = pin::pin!(async move {
            // --snip--
        });

        let rx_fut = pin::pin!(async {
            // --snip--
        });

        let tx_fut = pin::pin!(async move {
            // --snip--
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

        // 直接通过 await 去等待一个 future 时，Rust 会隐式地把它 pin 住
        trpl::join_all(futures).await;
    });
}
