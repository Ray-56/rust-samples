// Integer division without `panic!`
fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        // Failure is expressed as `None` value
        None
    } else {
        // The result Result is wrapped into the value of `Some`
        Some(dividend / divisor)
    }
}

// This function handles divisions that may fail
fn try_division(dividend: i32, divisor: i32) {
    // `Option` values ​​can be pattern matched, just like other enumeration types
    match checked_division(dividend, divisor) {
        None => println!("{} / {} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{} / {} = {}", dividend, divisor, quotient)
        },
    }
}

fn main() {
    try_division(4, 2);
    try_division(1, 0);

    // Binding `None` to a variable requires type annotation
    let none: Option<i32> = None;
    let _equivalent_none = None::<i32>;

    let optional_float = Some(0f32);

    // Unpacking `Some` will take out the wrapped value
    println!("{:?} unwraps to {:?}", optional_float, optional_float.unwrap());

    // Unpacking `None` will cause `panic!`
    println!("{:?} unwraps to {:?}", none, none.unwrap());
}