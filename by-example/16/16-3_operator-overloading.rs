use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the function of `+`. Here we implement `Add<Bar>`, which is used
// `trait` that adds an object and a right operand (RHS) of type `Bar`
// The following code block implements the operation `Foo + Bar = FooBar`
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we achieve addition that does not obey commutativity
// Here we implement `Add<Foo>`, which is a trait used to add objects and right operands of type `Foo`
// The following code block implements the operation `Bar + Foo = BarFoo`
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}