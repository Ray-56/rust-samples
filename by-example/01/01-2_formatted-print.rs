fn main() {
    println!("{} days", 31);

    // Without adding a suffix, 31 will automatically become the i32 type.
    // You can add suffixes to change the type of 31 (e.g. use 31i64 to declare 31 as i64 type)

    // There are many ways to write strings using variables
    // For example, you can use positional parameters
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named parameters can be used
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    // Special formats can be specified after `:`
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // You can right align text by a specified width
    // The following statement outputs "1", 5 spaces followed by 1
    println!("{number:>width$}", number=1, width=6);

    // You can add 0 to the left of the number, and the following statement will output "000001"
    println!("{number:>0width$}", number=1, width=6);

    // println! will check whether the number of parameters used is correct
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // Correction ^ Fill in the missing parameter: "James"

    // Create a structure containing a single `i32`. Named `Structure`
    #[derive(Debug)]
    struct Structure(i32);

    // But custom types like structs require a more complex way to handle
    // The following statement cannot be run
    println!("This struct `{:?}` won't print...", Structure(3));
    // Correction ^ output

    // Homework section
    println!("Pi is roughly {:.3}", 3.1415926);
}