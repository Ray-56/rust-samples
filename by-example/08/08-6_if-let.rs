fn main() {
    // All are of type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` structure reads: If `let` structures `number` into `Some(i)`, execute the statement block (`{}`)
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you want to specify a failure condition, use else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Deconstruction failed, switch to failure case
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an alternative condition in case of failure
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        // Deconstruction failed. Use `else if` to determine whether the conditions provided above are met
        println!("Didn't match a number. Lets go with a letter!");
    } else {
        // The condition evaluates to false. So the following is the default branch
        println!("I don't like letters. Let's go with an emoticon :)!");
    }


    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    // Create variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar, so nothing will be printed
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // The variable c matches Foo::Qux, which has a value, just like the Some() type in the example above.
    if let Foo:Qux(value) = c {
        println!("c is {}", value);
    }
}
