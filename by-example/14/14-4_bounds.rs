// 这个 trait 用来实现打印标记：`{:?}`
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

// 泛型`T`必须实现`Debug`。只要满足这点，无论什么类型都可以让下面函数正常工作
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T`必须实现`HasArea`。任意符合该约束的泛型的实例都可以访问`HasArea`的`area`函数
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, heigth: 4.0 };
    let _triangle = Triangle { length: 3.0, heigth: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    
}