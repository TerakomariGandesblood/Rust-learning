struct CustomSmartPointer {
    data: String,
}

// Drop trait 包含在 prelude 中
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("c: my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("d: other stuff"),
    };
    println!("CustomSmartPointers created.");
    // 提早清理
    // 不允许显式调用析构函数
    drop(_c);
    println!("CustomSmartPointer dropped before the end of main.");
}
