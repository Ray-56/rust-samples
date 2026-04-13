// Generate a constant with `'static` lifetime
static NUM: i32 = 18;

// Returns a reference to `NUM`, which does not take the `static` lifetime of `NUM`
// Instead, it is forced to be the same as the input parameter.
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        // Produce a `string` literal and print it
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // When `static_string` goes out of scope, the reference can no longer be used, but the data still exists in the binary file
    }

    {
        // Generate an integer for use by `coerce_static`
        let lifetime_num = 9;

        // Cast a reference to `NUM` to the lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}