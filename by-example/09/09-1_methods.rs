struct Point {
    x: f64,
    y: f64,
}

// Implemented code block, all methods of `Point` are given here
impl Point {
    // This is a static method
    // Static methods do not need to be called by the instance
    // This type of method is generally used as a constructor
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another static method that requires two parameters
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is an instance method
    // `&self` is syntactic sugar for `self: &Self`, where `Self` is the type of the method caller
    // In this example `Self`=`Rectangle`
    fn area(&self) -> f64 {
        // `self` accesses structure fields through the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` type method that returns the absolute value of the caller
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller to be mutable
    // `&mut self` is syntactic sugar for `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap-allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method will "consume" the caller's resources
    // `self` is syntactic sugar for `self: Self`
    fn destory(self) {
        // Deconstructing `self`
        let Pair(first, second) = self;

        println!("Destorying Pair({}, {})", first, second);

        // `first` and `second` are released after leaving the scope
    }
}

fn main() {
    let rectangle = Rectangle {
        // Static methods use double colons
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Instance methods are called using the dot operator
    // Note that the first parameter `&self` is passed implicitly, that is:
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Report an error! `rectangle` is immutable, but this method requires a mutable object
    // rectangle.translate(1.0, 0.0);
    // Try it ^ Uncomment this line

    // Running normally! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destory();

    // Report an error! The previous `destroy` call "consumed" the `pair`
    // pair.destory();
    // Try it ^ Uncomment this line
}