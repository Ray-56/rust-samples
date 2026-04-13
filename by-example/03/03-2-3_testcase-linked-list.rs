use List::*;

enum List {
    // Cons: Tuple structure, containing an element of the linked list and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: end node, indicating the end of the linked list
    Nil,
}

// Methods can be defined for enums
impl List {
    // Create an empty List instance
    fn new() -> List {
        // `Nil` is of type `List` (because the full name of `Nil` is `List:Nil`)
        Nil
    }

    // Process a List, insert a new element at its head, and return the List
    fn prepend(self, elem: u32) -> List {
        // `Cons` is also of type List
        Cons(elem, Box::new(self))
    }

    // Returns the length of the List
    fn len(&self) -> u32 {
        // `self` must be matched (match), because the behavior of this method depends on the value type of `self`
        // `self` is of type `&List`, `*self` is of type `List`, matching a specific `T` type is better than matching a reference `&T`
        match *self {
            // Cannot take ownership of tail because `self` is borrowed
            // So use a reference to tail
            Cons(_, ref tail) => 1 + tail.len(),
            // (Recursive) base case: an empty list of length 0
            Nil => 0,
        }
    }

    // Returns a string representation of the list (the string is heap allocated)
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap-allocated string.
                // Instead of printing the structure to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Append some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Display the last state of the linked list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}