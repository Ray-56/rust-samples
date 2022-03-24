fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec1 的`iter()`举出`&i32`类型
    let mut iter = vec.iter();
    // 对 vec2 的`into_iter()`举出`i32`类型
    let mut into_iter = vec.into_iter();

    // 对迭代器举出的元素引用时`&&i32`类型。解构成`i32`类型
    // 注意：`find`方法会把迭代器元素的引用传给闭包。
    // 迭代器元素自身是`&i32`类型，所以传给闭包的是`&&i32`类型
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // 对迭代器举出的元素的引用时`&i32`类型。解构成`i32`类型
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    
}