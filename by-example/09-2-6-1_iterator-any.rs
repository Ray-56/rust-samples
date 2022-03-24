fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 的 `iter()`举出`&i32`。（通过对`&x`匹配）把它解构成`i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的`into_iter()`举出的`i32`类型。无需解构
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组的`iter()`举出`&i32`
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 对数字的`into_iter()`通常举出`&i32`
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}