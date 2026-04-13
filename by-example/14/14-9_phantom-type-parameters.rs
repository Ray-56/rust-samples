use std::marker::PhantomData;

// This void group structure is generic to `A` and has a hidden parameter `B`
#[derive(PartialEq)] // Allow equality tests for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);

// This fake type struct is generic to `A` and has a hidden parameter `B`
#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> }

// Note: Storage space will be allocated for generic `A`, but not for `B`
// Therefore, `B` cannot participate in the operation

fn main() {
    // `f32` and `f64` here are hidden parameters
    // `PhantomTuple` type specified as `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // Specified as `<char, f64>``PhantomTuple` type
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'q',
        phantom: PhantomData,
    };

    // Compilation error! Type mismatch, so these values ​​cannot be compared
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);
}