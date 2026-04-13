

fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    /* let x: ! = panic!("This call never returns.");
    println!("You will never see this line!"); */

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Note that the return value of this match expression must be u32
            // Because the variable "addition" is of this type
            let addition: u32 = match i%2 == 1 {
                // The type of `i` variable is u32
                true => i,
                // On the other hand the `continue` expression does not return u32, it is still fine
                // Because it never returns, it does not violate the type requirement of the matching expression
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}