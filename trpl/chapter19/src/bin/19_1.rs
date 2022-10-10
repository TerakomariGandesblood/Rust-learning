use std::slice;

// 静态（static）变量，必须标注类型，静态变量只能储存拥有 'static 生命周期的引用
// 访问和修改可变静态变量都是不安全的
static HELLO_WORLD: &str = "hello, world!";

static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    // 裸指针（raw pointers）
    // 不能在不安全块之外解引用裸指针
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    // unsafe 提供了 5 个功能
    // 解引用裸指针
    // 调用不安全的函数或方法
    // 访问或修改可变静态变量
    // 实现不安全 trait
    // 访问 union 的字段
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);

        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// 关键字unsafe表示该函数具有调用时需要满足的要求，而 Rust 不会保证满足这些要求
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
// extern 块中声明的函数在 Rust 代码中总是不安全的
// "C" 部分定义了外部函数所使用的应用二进制接口（application binary interface，ABI）
extern "C" {
    fn abs(input: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
