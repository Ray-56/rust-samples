struct Droppable {
    name: &'static str,
}

// This simple `drop` implementation adds the ability to print to the console
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Drapping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // code block A
    {
        let _b = Droppable { name: "b" };

        // code block A
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variables can be destroyed manually using the `drop` function
    drop(_a);
    // Try it ^ comment out this line

    println!("end of the main function");

    // `_a` *will* not be destroyed again here because it has already been (manually) destroyed
}
