fn main() {
    // `n` 将在每次迭代中分别取 1, 2, ..., 100
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    /* for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    } */
    
    // for name in names.into_iter() {
    //     match name {
    //         "Ferris" => println!("There is a rustacean among us!"),
    //         _ => println!("Hello {}", name),
    //     }
    // }

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello"
        }
    }
    println!("names: {:?}", names);
}