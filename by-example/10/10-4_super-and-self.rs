fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::coll::function()`");
        }
    }

    pub fn indirect_call() {
        // Let us access all functions named `function` from this scope
        print!("called `my::indirect_call()`, that\n> ");

        // The `self` keyword represents the current module scope - in this case `my`
        // Calling `self::function()` and calling `function()` directly will give the same result
        // Because they represent the same function
        self::function();
        function();

        // You can also use `self` to access another module inside `my`
        self::cool::function();

        // The `super` keyword indicates negative scope (outside the `my` module)
        super::function();

        // This will bind `cool::function` within the *crate* scope
        // In this example, the crate scope is the outermost scope
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}