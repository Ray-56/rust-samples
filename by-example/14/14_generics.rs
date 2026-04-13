// a concrete type `A`
struct A;

// When defining type `Single`, `<A>` is not written before the first use of type `A`
// Therefore, `Single` is a concrete type, and `A` takes the above definition
struct Single(A);
// ^ Here is the first use of the type `Single`

// Here `<A>` appears before the first use of `T`, so `SingleGen` is a generic type
// Because `T` is generic, it can be of any type, including the concrete type `A` defined above
struct SingleGen<T>(T);

fn main() {
    // `Single` is a concrete type and explicitly uses type `A`
    let _s = Single(A);

    // Create a variable `_char` of type `SingleGen<char>` and set its value to `SingleGen('a')`
    // The type parameters of `SingleGen` here are explicitly specified.
    let _char: SingleGen<char> = SingleGen('a');

    // Type parameters of `SingleGen` can also be specified implicitly
    let _t = SingleGen(A); // Use `A` defined above
    let _i32 = SingleGen(6); // Use `i32` type
    let _char = SingleGen('a'); // Use `char`
}