use std::{collections::HashMap, thread, time::Duration};

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = 4;
    // 闭包周围的作用域被称为环境
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    // 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个（函数都实现了这三个 trait）
    // 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce
    // 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut
    // 而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn

    // NOTE https://zhuanlan.zhihu.com/p/23710601

    // 强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字，如将数据移动到新线程
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Self {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // 可以选择注明类型，不注明则需要调用
    // 如果闭包体只有一行则大括号可以省略
    // 每个闭包的类型都是不同的
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}
