

fn foo() -> ! {
    panic!("This call never returns.");
}

fn main() {
    /* let x: ! = panic!("This call never returns.");
    println!("You will never see this line!"); */

    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回值必须为 u32
            // 因为“addition”这个变量是这个类型
            let addition: u32 = match i%2 == 1 {
                // `i`变量的类型为 u32
                true => i,
                // 另一方面`continue`表达式不返回 u32，单它仍然没问题
                // 因为它永远不会返回，因此不会违犯匹配表达式的类型要求
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}