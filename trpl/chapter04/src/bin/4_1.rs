// 1、Rust 中的每一个值都有一个所有者（owner）
// 2、值在任一时刻有且只有一个所有者
// 3、当所有者（变量）离开作用域，这个值将被丢弃

fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(", World!");
        println!("{s}");
        // 离开作用域，调用 drop，释放 String 的内存
    }

    {
        let s1 = String::from("Hello");
        // move
        let _s2 = s1;
        // error: borrow of moved value: `s1`
        // println!("{}, {}", s1, s2);
    }

    #[allow(unused_assignments)]
    {
        let mut s = String::from("hello");
        // 立即调用 drop 并释放原始值的内存
        s = String::from("ahoy");
        println!("{s}, world!");
    }

    {
        let s1 = String::from("Hello");
        // copy
        let s2 = s1.clone();
        println!("{s1}, {s2}");
    }

    // 对于基本类型都实现了 Copy trait
    // 对于数组和 tuple 类型，如果内部元素类型都实现了 Copy trait，那么它也会自动实现 Copy trait
    // 对于 struct 和 enum 类型，不会自动实现 Copy trait
    // 注意 &T 实现了 Copy trait，但是 &mut T 没有实现
    // NOTE https://doc.rust-lang.org/std/marker/trait.Copy.html#impl-Copy-for-%26T
    {
        let arr1 = [1; 10];
        let arr2 = arr1;
        println!("{}", arr1[0]);
        println!("{}", arr2[0]);
    }

    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型实现 Copy trait
    // 类型实现了 Copy trait，默认将不是 move 而是 copy
    // NOTE https://zhuanlan.zhihu.com/p/21730929
}
