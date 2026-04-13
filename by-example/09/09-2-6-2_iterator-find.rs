fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vec1 enumerates `&i32` type
    let mut iter = vec.iter();
    // `into_iter()` for vec2 enumerates `i32` type
    let mut into_iter = vec.into_iter();

    // The reference to the element enumerated by the iterator is of type `&&i32`. Deconstructed into `i32` type
    // Note: The `find` method will pass a reference to the iterator element to the closure.
    // The iterator element itself is of type `&i32`, so what is passed to the closure is of type `&&i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    // The reference to the element enumerated by the iterator is of type `&i32`. Deconstructed into `i32` type
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    
}