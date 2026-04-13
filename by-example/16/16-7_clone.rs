// Unit structure without resources
#[derive(Debug, Clone, Copy)]
struct Nil;

// A struct containing resources that implements the `Clone` trait
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // Instantiate `Nil`
    let nil = Nil;
    // Copy `Nil`, no resources for move
    let copied_nil = nil;

    // Both `Nil` can be used independently
    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    // Bind `pair` to `moved_pair` and move the resource
    let moved_pair = pair;
    println!("copy: {:?}", moved_pair);

    // Report an error! `pair` has lost its resources
    // println!("original: {:?}", pair);
    // Try it ^ Uncomment this line

    // Clone `moved_pair` (including its resources) to `cloned_pair`
    let cloned_pair = moved_pair.clone();
    // Use std::mem::drop to destroy the original pair
    drop(moved_pair);

    // Report an error! `moved_pair` has been destroyed
    // println!("copy: {:?}", moved_pair);
    // Try it ^ Uncomment this line

    // The results from.clone() are still available
    println!("clone: {:?}", cloned_pair);
}