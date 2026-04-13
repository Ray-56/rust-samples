struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed by reference or primitive type
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Report an error! `point` cannot be borrowed mutably because there are currently immutable borrows
    // let mutable_borrow = &mut point;
    // TODO ^ Try uncommenting this line

    // The borrowed value is reused here
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // The immutable reference is no longer used by the rest of the code, so it can be re-borrowed using a mutable reference
    let mutable_borrow = &mut point;

    // Modify data through mutable references
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Report an error! `point` cannot be borrowed immutably because it is currently borrowed mutably
    // let y = &point.y;
    
    // Report an error! Unable to print because `println!` uses an immutable reference
    // println!("Point Z coordinate is {}", point.z);

    // Running normally! Mutable references can be passed into `println!` as immutable types
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z,
    );

    // The mutable reference is no longer used in the rest of the code, so it can be re-borrowed
    let new_borrowed_point = &point;
    println!(
        "Point now has corrdinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}
