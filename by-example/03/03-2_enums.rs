// 该属性用于隐蔽对未使用代码的警告
#![allow(dead_code)]

// 创建一个`enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了`enum`
// 取值的种类：`PageLoad`不等于`PageUnload`，`KeyPress(char)`不等于`Paste(String)`。各个取值不同，相互独立
enum WebEvent {
    // 一个`enum`可以是单元结构体（称为`unit-like`或`unit`）
    PageLoad,
    PageUnload,
    // 或者一个元组结构体
    KeyPress(char),
    Paste(String),
    // 又或者一个普通的结构体
    Click { x: i64, y: i64 }
}

// 此函数将一个`WebEvent`enum 作为参数，无返回值
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从`enum`里解构出`c`
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把`Click`解构给`x`and`y`
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()`从一个字符串切片中创建一个具有所有权的`String`
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}