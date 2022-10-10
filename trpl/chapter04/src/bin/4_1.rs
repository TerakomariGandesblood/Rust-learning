fn main() {
    {
        let mut s = String::from("Hello");
        s.push_str(", World!");
        println!("{}", s);
        // 离开作用域，调用 drop，释放 String 的内存
    }

    {
        let s1 = String::from("Hello");
        // move
        let _s2 = s1;
        // error: borrow of moved value: `s1`
        // println!("{}, {}", s1, s2);
    }

    {
        let s1 = String::from("Hello");
        // copy
        let s2 = s1.clone();
        println!("{}, {}", s1, s2);
    }

    {
        let arr1 = [1; 10];
        let arr2 = arr1;
        println!("{}", arr1[0]);
        println!("{}", arr2[0]);
    }

    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
    // 类型实现了 Copy trait，默认将不是 move 而是 copy，见
    // https://zhuanlan.zhihu.com/p/21730929
}
