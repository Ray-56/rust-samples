// 一个具体类型`A`
struct A;

// 在定义类型`Single`时，第一次使用类型`A`之前没有写`<A>`
// 因此，`Single`是一个具体的类型，`A`取上面的定义
struct Single(A);
//            ^ 这里是`Single`对类型的第一次使用

// 此处`<A>`在第一次使用`T`前出现，所以`SingleGen`是一个泛型类型
// 因为`T`是泛型的，所以它可以是任何类型，包括上面定义的具体类型`A`
struct SingleGen<T>(T);

fn main() {
    // `Single`是具体的类型，并且显示的使用类型`A`
    let _s = Single(A);

    // 创建一个`SingleGen<char>`类型的变量`_char`，并令其值为`SingleGen('a')`
    // 这里的`SingleGen`的类型参数是显示指定的
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`的类型参数也可以隐式地指定
    let _t = SingleGen(A); // 使用在上面定义的`A`
    let _i32 = SingleGen(6); // 使用`i32`类型
    let _char = SingleGen('a'); // 使用`char`
}