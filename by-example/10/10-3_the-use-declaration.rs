// Bind `deep::nested::function` path to `other_function`
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

fn main() {
    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`
        // This `function()` will shadow the outer function of the same name
        use deeply::nested::function;
        function();

        // `use` bindings have local scope. In this example, the shadow of `function()` only exists in this code block
        println!("Leaving block");
    }

    function();
}