macro_rules! create_function {
    // 此宏接受一个`ident`指示符表示的参数，并创建一个名为`$func_name`的函数
    // `ident`指示符用于变量名或函数名
    ($func_name: ident) => {
        fn $func_name() {
            // `stringify!`宏把`ident`转换成字符串
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

// 借助上述宏来创建名为`foo`和`bar`的函数
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression: expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression,)
    };
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // 回想一下，代码块也是表达式
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}
