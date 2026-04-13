use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // If the value already exists, then `HashSet::insert()` returns false
    // assert!(b.insert(4), "Value 4 is already in set B!");

    b.insert(5);

    // If the element type of a collection implements `Debug`, then the collection also implements `Debug`
    // This usually prints the elements in the format `[elem1, elem2,...]`
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // Print out of order [1, 2, 3, 4, 5]
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // This will print out [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // Print out of order [2, 3, 4]
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // print [1, 5]
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}