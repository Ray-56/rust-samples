#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit structure
struct Unit;

// tuple structure
struct Pair(i32, f32);

// Structure with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// A structure can be a field of another structure
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by giving the positions of the upper left and lower right corners in space in the namespace.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Use simple writing to initialize fields and create structures
    let name = String::from("Ray");
    let age = 27;
    let ray = Person { name, age };

    // Print the structure in Debug mode
    println!("{:?}", ray);

    // Instantiate structure `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Create a new Point using structure update syntax
    // In this way, the fields of the previous point can be used
    let bottom_right = Point { x: 5.2, ..point };

    // `new_point.y` is the same as `point.y`, because this field comes from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructuring point using `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // The instantiation of a structure is also an expression
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit structure
    let _unit = Unit;

    // Instantiate a tuple structure
    let pair = Pair(1, 0.1);

    // Access fields of tuple structure
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructuring a tuple structure
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Homework section
    fn rect_area(r: Rectangle) -> f32 {
        let Rectangle {
            top_left: Point { x: top, y: left },
            bottom_right: Point {
                x: bottom,
                y: right,
            },
        } = r;

        (right - left) * (bottom - top)
    }
    println!("rect_area() => {}", rect_area(_rectangle));

    fn square(p: Point, x: f32) -> Rectangle {
        let Point { x: top, y: left } = p;
        Rectangle {
            top_left: p,
            bottom_right: Point {
                x: top + x,
                y: left + x,
            }
        }
    }
    println!("square() => {:?}", square(Point { x: 5., y: 10. }, 5.))
}
