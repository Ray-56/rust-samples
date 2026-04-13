struct Container(i32, i32);

// This trait checks whether the given 2 items are stored in the container
// and be able to get the first or last value of the container
trait Contains {
    // Generic types that can be used by methods are defined here
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // Indicate what types `A` and `B` are. If the `input` type is `Container(i32, i32)`,
    // Then the `output` (output) type will be determined as `i32` and `i32`
    type A = i32;
    type B = i32;

    // `&Self::A` and `&Self::B` are also legal types here
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) -> i32 { self.0 }
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}