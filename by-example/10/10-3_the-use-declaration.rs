// 将`deep::nested::function`路径绑定到`other_function`
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
    // 更容易访问到`deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // 这和`use deeply::nested::function as function`等价
        // 此`function()`将遮蔽外部的同名函数
        use deeply::nested::function;
        function();

        // `use`绑定拥有局部作用域。这个例子中，`function()`的遮蔽只存在这个代码块中
        println!("Leaving block");
    }

    function();
}