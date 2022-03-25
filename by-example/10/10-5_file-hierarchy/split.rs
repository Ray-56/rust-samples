// 此声明会查找名为`my.rs`或者`my/mod.rs`的文件，并将该文件的内容放到此作用域中一个名为`my`的模块里面
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