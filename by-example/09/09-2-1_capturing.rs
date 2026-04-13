fn main() {
    use std::mem;

    let color = String::from("green");

    // This closure prints `color`. It immediately borrows (by reference, `&`) `color` and stores that borrow and the closure itself in
    // in the `print` variable. `color` will remain borrowed until `print` goes out of scope
    // 
    // `println!` only needs to be passed by reference to be used, and this closure also captures the reference of the variable, so `println!` can be used without further processing.
    let print = || println!("`color`: {}", color);

    // Using borrowing to call closure `color`
    print();

    // `color` can be immutably borrowed again because the closure only holds an immutable reference to `color`
    let _reborrow = &color;
    print();

    // Moving or re-borrowing is allowed after the last use of `print`
    let _color_moved = color;

    let mut count = 0;
    // This closure is the `count` value being incremented. To do this, it needs to get `&mut count` or `count` itself
    // But the requirement of `&mut count` is not so strict, so we adopt this approach. The closure immediately borrows `count`
    // 
    // `mut` needs to be added before `inc`, because a `&mut` variable is stored in the closure. When calling a closure,
    // The change of this variable means that the internal changes of the closure have occurred. So closures need to be mutable
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Calling closures using mutable borrow
    inc();

    // Because the closure is called later, `count` is still mutably borrowed
    // Attempting to re-borrow will result in an error
    // let _reborrow = &count;
    // ^ Try it: uncomment this line
    inc();

    // The closure no longer borrows `&mut count` and therefore re-borrows correctly
    let _count_reborrowed = &mut count;

    // non-copy type
    let movable = Box::new(3);

    // `mem::drop` requires the `T` type itself, so the closure will capture the value of the variable. In this case,
    // The copyable type will be copied to the closure so that the original value is not affected. Non-copyable types must be moved
    // (move) into the closure, so the `movable` variable is immediately moved to the closure here.
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` consumes the variable, so the closure can only be called once
    consume();
    // consume();
    // ^ Try it: uncomment this line
}