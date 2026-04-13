use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first parameter is the path to call this program
    println!("My path is {}.", args[0]);

    // The remaining arguments are command line arguments passed to the program
    // Please call the program like this
    //      $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}", args.len() - 1, &args[1..]);
}