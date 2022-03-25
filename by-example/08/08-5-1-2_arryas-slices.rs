fn main() {
    // 尝试更改数组中的值，或将它设置为切片
    let array = [33, -2, 6];

    match array {
        // 将第二个和第三个元素绑定到各自的变量
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // 可以用`_`忽略单个值
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        // 也可以绑定一些并且忽略其余的
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // 下面的代码会报错
        // [-1, second] => println!("This is a test"),
        // 或者将它们存储在另一个数组或切片中（类型取决与匹配的值）
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        // 结合这些模式，可以绑定第一个和最后一个，并将其余的值存储在一个数组中
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        )
    }
}