// `NanoSecond` is the new name of `u64`
type NanoSecond = u64;
type Inch = u64;

// Disable warnings with this property
#[allow(non_camel_case_types)]
type u64_t = u64;
// Try it ^ Remove the attribute above

fn main() {
    // `NanoSecond` = `Inch` = `u64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    // Note that type aliases do not provide additional type safety because aliases are not new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}