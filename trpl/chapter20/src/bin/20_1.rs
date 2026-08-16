// unsafe 提供了 5 个功能
// 解引用裸指针
// 调用不安全的函数或方法
// 访问或修改可变静态变量
// 实现不安全 trait
// 访问 union 的字段

use std::slice;

// 静态（static）变量，必须标注类型，必须使用常量表达式初始化
// 访问和修改可变静态变量都需要 unsafe
static HELLO_WORLD: &str = "hello, world!";

static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    // 裸指针（raw pointers）
    // 不可变意味着指针解引用之后不能直接赋值
    // https://blog.rust-lang.org/2024/10/17/Rust-1.82.0/#native-syntax-for-creating-a-raw-pointer
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);

        dangerous();
    }

    let mut v = [1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("Absolute value of -3 according to C: {}", abs(-3));

    println!("name is: {HELLO_WORLD}");

    // 不允许创建一个可变静态变量的引用
    // unsafe {
    //     println!("COUNTER: {COUNTER}");
    // }

    add_to_count(3);
    unsafe {
        COUNTER += 1;
    }
}

// 关键字 unsafe 表示该函数具有调用时需要满足的要求，而 Rust 不会保证满足这些要求
// 不安全函数体也是有效的 unsafe 块，不需要再使用 unsafe
unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// extern，创建和使用外部函数接口（Foreign Function Interface，FFI）
// "C" 部分定义了外部函数所使用的应用二进制接口（application binary interface，ABI）
// ABI 定义了如何在汇编语言层面调用此函数。"C" ABI 是最常见的，并遵循 C 编程语言的 ABI
unsafe extern "C" {
    // unsafe extern 中声明的任何项都隐式地是 unsafe 的
    // safe 关键字来表明这个特定的函数即便是在 unsafe extern 块中也是可以安全调用的
    safe fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// 如果实现了一个包含一些不是 Send 或 Sync 的类型，比如裸指针，并希望将此类型标记为 Send 或
// Sync，则必须使用 unsafe Rust 不能验证我们的类型保证可以安全的跨线程发送或在多线程间访问，
// 所以需要我们自己进行检查并通过 unsafe 表明
#[allow(clippy::missing_safety_doc)]
unsafe trait Foo {}
unsafe impl Foo for i32 {}

// 访问 union 中的字段需要 unsafe，主要用于和 C 代码中的 union 交互
