fn main() {
    // 获得一个`i32`类型的引用。`&`表示取引用
    let reference = &4;

    match reference {
        // 如果用`&val`这个模式去匹配`reference`，就相当于做这样的比较
        // `&i32` `&val` 我们看到，如果去掉匹配的`&`，`i32`应当赋给`val`
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果不想用`&`，需要在匹配前解引用
    match *reference {
        val => println!("Got a value via destructuring: {:?}", val),
    }

    // 如果一开始就不用引用，会怎样？`reference`是一个`&`类型，因为赋值语句
    // 的右边已经是一个引用。单下面这个不是引用，因为右边不是
    let _not_a_reference = 3;

    // Rust 对这种情况提供了`ref`。它更改了赋值行为，从而可以对具体值创建引用
    // 下面这行将得到一个引用
    let ref _is_reference = 3;

    // 相应的，定义两个非引用的变量，通过`ref`和`ref mut`仍可取得其引用
    let value = 5;
    let mut mut_value = 6;

    // 使用`ref`关键字来创建引用
    match value {
        ref r => println!("Got a reference to value: {:?}", r),
    }

    // 类似地使用`ref mut`
    match mut_value {
        ref mut m => {
            // 以及获得了`mut_value`的引用，先要解引用，才能改变它的值
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}