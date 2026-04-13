trait UsernameWidget {
    // Get the selected username from this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age from this widget
    fn get(&self) -> u8;
}

// Form with both UsernameWidget and AgeWidget
struct Form {
    usename: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.usename.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        usename: "rustacean".to_owned(),
        age: 28,
    };

    // If you uncomment the following line, you will receive an error message saying "multiple `get` found" (Multiple `get` found)
    // Because after all, there are multiple methods named `get`
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}