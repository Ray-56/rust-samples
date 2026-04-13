fn main() {
    // (All type annotations are not required)
    // A reference to a string allocated in read-only memory
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    // Iterate words in reverse order, no new strings are allocated here
    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // Assign characters to a vector, sort and remove duplicates
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // Create an empty and growable `String`
    let mut string = String::new();
    for c in chars {
        // Insert a character at the end of the string
        string.push(c);
        // Insert a string at the end of the string
        string.push_str(", ");
    }

    // The shortened string is a slice of the original string, so no new allocations are performed
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocates a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}