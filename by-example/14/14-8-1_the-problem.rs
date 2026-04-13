struct Container(i32, i32);

// This trait checks whether the given 2 items are stored in the container
// and be able to get the first or last value of the container
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly require `A` and `B`
    fn first(&self) -> i32; // Unshown requirement `A` or `B`
    fn last(&self) -> i32; // Unshown requirement `A` or `B`
}

impl Contains<i32, i32> for Container {
    // True if the stored number is equal to the given one
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // get the first number
    fn first(&self) -> i32 {
        self.0
    }

    // get the last number
    fn last(&self) -> i32 {
        self.1
    }
}

// The container `C` contains types `A` and `B`. In view of this, it must be pointed out that `A` and `B` appear to be very troublesome
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
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
