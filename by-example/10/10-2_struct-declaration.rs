mod my {
    // A public structure with a common field (type generic `T`)
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public structure with a private field (of type generic `T`)
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // a public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    // A public structure with public fields that can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be accessed normally
    println!("The open box contains: {}", open_box.contents);

    // Public structures with private fields cannot be constructed using field names
    // Report an error! `ClosedBox` contains private fields
    // let closed_box = my::ClosedBox { contents: "classified information" };
    // Try it ^ Uncomment this line

    // However, structures with private fields can be created using public constructors.
    let _closed_box = my::ClosedBox::new("classified information");

    // And private fields in a structure cannot be accessed
    // Report an error! The `contents` field is private
    // println!("The closed box contains: {}", _closed_box.contents);
}