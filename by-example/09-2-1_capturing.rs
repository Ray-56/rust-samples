fn main() {
    use std::mem;

    let color = String::from("green");

    // 这个闭包打印`color`。它会立即借用（通过引用，`&`）`color`并将该借用和闭包本身存储到
    // `print`变量中。`color`会一直保持被借用状态知道`print`离开作用域
    // 
    // `println!`只需要传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需进一步处理就可以使用`println!`
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包`color`
    print();

    // `color`可再次被不可变借用，因为闭包只持有一个指向`color`的不可变引用
    let _reborrow = &color;
    print();

    // 在最后使用`print`之后，移动或重新借用都是允许的
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包是`count`值增加。要做到这点，它需要得到`&mut count`或者`count`本身
    // 但`&mut count`要求没那么严格，所以我们采取这种方式。该闭包立即借用`count`
    // 
    // `inc`前面需要加上`mut`，因为闭包内存储着一个`&mut`变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发成了变化。因此闭包需要是可变的
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用`count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉
    inc();

    // 闭包不在借用`&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）
    let movable = Box::new(3);

    // `mem::drop`要求`T`类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而`movable`变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume`消耗了该变量，所以该闭包只能调用一次
    consume();
    // consume();
    // ^ 试一试：将此行注释去掉
}