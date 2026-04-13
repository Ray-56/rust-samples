// This statement will look for a file named `my.rs` or `my/mod.rs` and place the contents of the file into a module named `my` in this scope
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();
    
    my::nested::function();
}