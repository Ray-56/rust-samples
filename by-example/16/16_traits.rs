struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Static method signature: `Self` represents the implementor type (implementor type)
    fn new(name: &'static str) -> Self;

    // Instance method signature: These methods will return a string
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementers can use its trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait on `Sheep`
impl Animal for Sheep {
    // `Self` is the implementer type: `Sheep`
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    // Default trait methods can be overloaded
    fn talk(&self) {
        // For example, we can add some quiet contemplation
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    // This situation requires type annotation
    let mut dolly: Sheep = Animal::new("Dolly");
    // Try it ^ Remove type annotations

    dolly.talk();
    dolly.shear();
    dolly.talk();
}