#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
#[derive(Debug)]
struct Rectangle {
    // 可以在命名空间给定左上角和右下角在空间中的位置来指定矩形
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // 使用简单的写法初始化字段，并创建结构体
    let name = String::from("Ray");
    let age = 27;
    let ray = Person { name, age };

    // 以 Debug 方式打印结构体
    println!("{:?}", ray);

    // 实例化结构体 `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的 Point
    // 这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, ..point };

    // `new_point.y`与`point.y`一样，因为这个字段就是从`point`中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // 使用`let`绑定来解构 point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 结构体的实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // 作业部分
    fn rect_area(r: Rectangle) -> f32 {
        let Rectangle {
            top_left: Point { x: top, y: left },
            bottom_right: Point {
                x: bottom,
                y: right,
            },
        } = r;

        (right - left) * (bottom - top)
    }
    println!("rect_area() => {}", rect_area(_rectangle));

    fn square(p: Point, x: f32) -> Rectangle {
        let Point { x: top, y: left } = p;
        Rectangle {
            top_left: p,
            bottom_right: Point {
                x: top + x,
                y: left + x,
            }
        }
    }
    println!("square() => {:?}", square(Point { x: 5., y: 10. }, 5.))
}
