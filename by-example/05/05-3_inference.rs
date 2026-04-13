fn main() {
    // Because of the type declaration, the compiler knows that the type of `elem` is u8
    let elem = 5u8;

    // Create an empty vector (vector, an array of variable length that can grow)
    let mut vec = Vec::new();
    // Now the compiler doesn't know the specific type of `vec`, it only knows that it is a vector of something (`vec<_>`)

    // Insert `elem` into vector
    vec.push(elem);
    // Now the compiler knows that `vec` is a vector of u8 (`Vec<u8>`)
    
}