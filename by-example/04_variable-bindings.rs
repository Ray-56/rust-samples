fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将`an_iteger`复制到`copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet to unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}