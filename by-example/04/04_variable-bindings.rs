fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // Copy `an_iteger` to `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet to unit value: {:?}", unit);

    // The compiler will generate a warning about unused variable bindings; you can prefix the variable name with an underscore to suppress the warning.
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
}