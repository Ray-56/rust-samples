fn main() {
    // Try changing the values ​​in the array, or setting it to a slice
    let array = [33, -2, 6];

    match array {
        // Bind the second and third elements to their respective variables
        [0, second, third] =>
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        // Single values ​​can be ignored using `_`
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        // It is also possible to bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The following code will report an error
        // [-1, second] => println!("This is a test"),
        // Or store them in another array or slice (type depends on matching values)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        // Combining these patterns it is possible to bind the first and last and store the remaining values ​​in an array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        )
    }
}