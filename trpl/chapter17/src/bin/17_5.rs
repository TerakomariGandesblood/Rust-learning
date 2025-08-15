// use std::mem;
// use std::pin::Pin;
// use std::task::Context;

// pub enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// pub trait Future {
//     type Output;

//     // 对于大部分功能，调用者不应在 future 返回 Ready 后再次调用 poll
//     // 很多 future 在完成后再次轮询会 panic
//     // 使用 await 的代码时，Rust 会在底层将其编译为调用 poll 的代码
//     // 运行时 poll 其所负责的每一个 future，在它们还没有完成时使其休眠

//     // self 的类型注解与其它参数的类型注解类似，但有两个关键区别
//     // 它告诉 Rust 在调用该方法时 self 必须具备的类型
//     // 它不能是任意类型。是一个该类型的引用或者智能指针，或者一个封装了该类型引用的 Pin

//     // Pin 是一个类指针类型的封装，适用于实现了 Deref 或 DerefMut trait 的类型
//     // 绝大多数类型都不在意是否被移动，自动实现了 Unpin trait
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }

// fn main() {
//     let mut sr_1 = SelfReference::new("Hello");
//     // 是一个 unsafe 函数，这是因为需要使用者自己遵守约定只使用 Pin 提供的 api
// 来获取并使用可变引用     let sr_1 = unsafe { Pin::new_unchecked(&mut sr_1) };
//     sr_1.init();

//     let mut sr_2 = SelfReference::new("World");
//     let sr_2 = unsafe { Pin::new_unchecked(&mut sr_2) };
//     sr_2.init();

//     println!("Before swap:");
//     println!(
//         "sr_1: {{ a: {}, b: {} }}",
//         sr_1.as_ref().get_a(),
//         sr_1.as_ref().get_b()
//     );
//     println!(
//         "sr_2: {{ a: {}, b: {} }}",
//         sr_2.as_ref().get_a(),
//         sr_2.as_ref().get_b()
//     );

//     println!("If we want to swap:");
//     // 如果没有实现 Unpin，则无法获取可变引用
//     mem::swap(sr_1.get_mut(), sr_2.get_mut());
// }

// #[derive(Debug)]
// struct SelfReference {
//     a: String,
//     b: *const String,
//     // c: PhantomPinned,
// }

// impl SelfReference {
//     fn new(msg: &str) -> Self {
//         Self {
//             a: msg.to_string(),
//             b: std::ptr::null(),
//             // c: PhantomPinned::default(),
//         }
//     }

//     fn init(self: Pin<&mut Self>) {
//         let ptr_to_a = &self.a as *const _;
//         unsafe {
//             self.get_unchecked_mut().b = ptr_to_a;
//         }
//     }

//     fn get_a(self: Pin<&Self>) -> &str {
//         &self.get_ref().a
//     }

//     fn get_b(self: Pin<&Self>) -> &str {
//         unsafe { &*self.b }
//     }
// }
fn main() {}
