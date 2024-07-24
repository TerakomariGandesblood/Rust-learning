// Send 标记 trait 表明实现了 Send 的类型值的所有权可以在线程间传送
// 几乎所有的 Rust 类型都是 Send 的，不过有一些例外，包括 Rc<T>
// 任何完全由 Send 的类型组成的类型也会自动被标记为 Send。
// 几乎所有基本类型都是 Send 的，除了裸指针（raw pointer）

// Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用
// 对于任意类型 T，如果 &T 是 Send 的话 T 就是 Sync 的
// 基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的
// 智能指针 Rc<T> 不是 Sync 的
// RefCell<T> 和 Cell<T> 系列类型不是 Sync 的。
// RefCell<T> 在运行时所进行的借用检查也不是线程安全的
// Mutex<T> 是 Sync 的

// 手动实现 Send 和 Sync 是 unsafe 的

// https://github.com/pretzelhammer/rust-blog/blob/master/posts/translations/zh-hans/tour-of-rusts-standard-library-traits.md#send--sync
// See https://zhuanlan.zhihu.com/p/24142191
fn main() {}
