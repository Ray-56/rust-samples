#[derive(Clone, Copy)]
struct Point { x: i32, y: i32 }

fn main() {
    let c = 'Q';

    // The `ref` keyword on the left in the assignment statement is equivalent to the `&` symbol on the right
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // `ref` also works when destructuring a struct
    let _copy_if_x = {
        // `ref_to_x` is a reference to the `x` field of `point`
        let Point { x: ref ref_to_x, y: _ } = point;

        // Returns a copy of the `x` field of `point`
        *ref_to_x
    };

    // mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be combined with `mut` to create mutable references
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // Change the field `y` of `mutable_point` through a mutable reference
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // mutable tuple containing a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructuring `mutable_tuple` to change the value of `last`
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }
    println!("tuple is {:?}", mutable_tuple);
}