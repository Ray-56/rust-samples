fn main() {
    // Variables can be given type specifications
    let logical: bool = true;

    let a_float: f64 = 1.0; // General instructions
    let a_integer = 5i32; // Suffix description

    // Otherwise, the type will be determined by default
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // Types are also automatically inferred based on context
    let mut inferred_type = 12; // The i64 type is inferred based on the assignment on the next line
    inferred_type = 4294967296i64;

    // A mutable variable whose value can be changed
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Report an error! The type of the variable cannot be changed
    // mutable = true;

    // But you can use shadow to overwrite the previous variables
    let mutable = true;
}