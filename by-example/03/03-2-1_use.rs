// This attribute is used to hide warnings about unused code
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Exposed `use` names make them directly available without specifying the `Status` they came from
    use Status::{Poor, Rich};
    // Automatic `use` `Work` internal names
    use Work::*;

    // `Poor` is equivalent to `Status::Poor`
    let status = Poor;
    // `Civilian` is equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        // Note that the full path is not used here because `use` is explicitly used above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Notice again that the full path is not used
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}