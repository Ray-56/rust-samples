fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    // `name` is removed from person, but `age` is just a reference
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Report an error! Borrowing of partial move values: Partial borrowing of `person` results
    // println!("The person struct is {:?}", person);

    // `person` cannot be used, but `person.age` can continue to be used because it has not been moved.
    println!("The person's age from person struct is {}", person.age);
}