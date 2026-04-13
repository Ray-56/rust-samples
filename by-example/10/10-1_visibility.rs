// A module named `my_mod`
mod my_mod {
    // Items in modules have private visibility by default
    fn private_function() {
        println!("called `my_mod::private_function`");
    }

    // Use the `pub` modifier to change the default visibility
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items within the same module, even if it is private
    pub fn indirect_access() {
        print!("called `my_mod::indirect_acess()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function`");
        }

        // Functions defined using the `pub(in path)` syntax are only visible in the given path
        // `path` must be a parent module or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n ");
            public_function_in_nested();
        }

        // Functions defined using the `pub(self)` syntax are only visible in the current module
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested`");
        }

        // Functions defined using the `pub(super)` syntax are only visible in the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` makes the function visible only in the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // Visibility of nested modules follows the same rules
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // The module mechanism disambiguates items with the same name
    function();
    my_mod::function();

    // Public items, including those within nested modules, can be accessed outside the parent module
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be accessed from anywhere in the same crate
    my_mod::public_function_in_crate();
}