fn main() {
    // 变量绑定
    let x = 5;

    // 表达式
    x;
    x + 1;
    15;

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将次表达式赋给`y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将`()`赋给`z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}