// 此函数全的一个 box 的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 此函数借用了一个 i32 类型
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // 创建一个存在堆中的 i32 类型，以及一个存在栈中的 i32 类型
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了 box 的内容，但没有取得所有权，所以 box 的内容之后可以再次借用
    // 注意：请注意函数自身就是一个作用域，因此下面的两个函数运行完成以后在函数中临时创建的引用也就不复存在了
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 取得一个对 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 报错！当`boxed_i32`里面的值在之后的作用域中被借用时，不能将其销毁
        // eat_box_i32(boxed_i32);
        // 改正 ^ 注释掉此行

        // 在`_ref_to_i32`里面的值被销毁后，尝试借用`_ref_to_i32`
        // （注意：如果此处不借用，则在上一行的代码中，eat_box_i32(boxed_i32) 可以将`boxed_i32`销毁）
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32`离开作用域且不再被借用
    }

    // `boxed_i32`现在可以将所有权交给`eat_i32`并销毁
    // （注意：能够销毁时因为已经不存在对`boxed_i32`的引用）
    eat_box_i32(boxed_i32);
}