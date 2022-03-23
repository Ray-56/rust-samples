use std::fmt;

// 元组可以充当函数的参数和返回值
fn reverse(pair:(i32, bool)) -> (bool, i32) {
    // 可以使用`let`吧一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 在“动手试一试”的练习中要用到下面这个结构体
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面的两行注释，阅读编译器给出的错误提示
    // Printing tuples is currently implemented using a macro that only works up to 12 elements
    // ^ from stackoverflow, https://stackoverflow.com/questions/51846320/why-is-tuple-formatting-limited-to-12-items-in-rust

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量做区分
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    // 以下为作业部分

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} {} )", self.0, self.1)?;
            write!(f, "\n")?;
            write!(f, "( {} {} )", self.2, self.3)
        }
    }
    println!("Matrix: \n{}", matrix);

    fn transpose(matrix: Matrix) -> Matrix {

        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }
    println!("Transpose: \n{}", transpose(matrix));
}