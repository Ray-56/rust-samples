use std::fmt::Debug; // Traits for constraints

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T`, where `T` has a one-position lifetime `'a`
// `T` has a lifetime limit, any *reference* in `T` must live longer than `'a`
// In addition, the life cycle of `Ref` cannot exceed `'a`

// A generic function that uses the `Debug` trait to print content
fn print<T>(t: T)
where
    T: Debug,
{
    println!("`print`: t is {:?}", t);
}

// This accepts a reference to `T`, where `T` implements the `Debug` trait, and all *references* in `T` must outlive `'a`.
// In addition, `'a` also lives longer than the function.
fn print_ref<'a, T>(t: &'a T)
where
    T: Debug,
{
    println!("`print_ref`: t is {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}
