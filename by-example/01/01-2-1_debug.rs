// 推导`Structure`的`fmt::Debug`实现
// `Structure`是一个包含单个`i32`的结构体
#[derive(Debug)]
struct Structure(i32);

// 将`Structure`放到结构体`Deep`中。然后使`Deep`也能够打印
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Persion<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // 使用`{:?}`打印和使用`{}`类似
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure`也可以打印
    println!("Now {:?} will print!", Structure(3));

    // 使用`derive`的一个问题是不能控制输出的形式
    // 加入我只想展示一个`7`怎么办
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Ray";
    let age = 29;
    let ray = Persion { name, age };

    // 美化打印
    println!("{:#?}", ray);
}
