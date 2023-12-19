// Rust 通过在编译时进行泛型代码的单态化（monomorphization）来保证效率。
// 单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。

// 类型参数声明位于函数名称与参数列表中间的尖括号 <> 中
fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 必须在结构体名称后面的尖括号中声明泛型参数的名称
struct Point<T> {
    _x: T,
    _y: T,
}

// 必须在 impl 后面声明 T，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型
impl<T> Point<T> {
    fn _x(&self) -> &T {
        &self._x
    }
}

// 为特定类型实现
impl Point<f32> {
    fn _distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

enum _Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let list = vec![1, 2, 3, 304, 100];
    println!("{}", largest(&list));

    let _integer = Point { _x: 1, _y: 2 };
}
