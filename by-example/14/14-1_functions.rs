struct A; // Concrete type `A`
struct S(A); // Concrete type `s`
struct SGen<T>(T); // Generic type `SGen`

// All the following functions take ownership of the variable and immediately take it out of scope and release the variable.

// Define a function `ref_fn` that accepts a parameter `_s` of type `S`
// Because there is no generic type parameter like `<T>`, this is not a generic function
fn reg_fn(_s: S) {}

// Define a function `gen_spec_t` that accepts a parameter `_s` of type `SGen<A>`
// `SGen<>` explicitly accepts the type parameter `A`, and in `gen_spec_t`, `A` is not used as a generic type parameter, so the function is not generic
fn gen_spec_t(_s: SGen<A>) {}

// Define a function `gen_spec_i32` that accepts a parameter `_s` of type `SGen<i32>`
// `SGen<>` explicitly accepts the type parameter `i32`, and `i32` is a concrete type
// Since `i32` is not a generic type, this function is not generic either
fn gen_spec_i32(_s: SGen<i32>) {}

// Define a function `generic` that accepts a `SGen<T>` type parameter `_s`
// Because there is `<T>` before `SGen<T>`, this function is a generic function about `T`
fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Use non-generic functions
    reg_fn(S(A)); // concrete type
    gen_spec_t(SGen(A)); // Implicitly specifying type parameter `A`
    gen_spec_i32(SGen(6)); // Implicitly specifying type parameter `i32`

    // Specified type parameter `char` displayed for `generic()`
    generic::<char>(SGen('a'));

    // Implicitly specifying type parameter `char` for `generic()`
    generic(SGen('c'));
}