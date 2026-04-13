fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // Exemplify `&i32` for `iter()` of vec. Deconstruct it into `i32` (by matching `&x`)
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // The `i32` type enumerated by `into_iter()` on vec. No need to deconstruct
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // Execute `&i32` for `iter()` of an array
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for numbers usually enumerates `&i32`
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}