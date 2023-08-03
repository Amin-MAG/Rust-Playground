// Rust provides three primary collection
// types in its standard library: Vector,
// HashMap, and Hashset

use std::collections::{HashMap, HashSet};

// Vec (Vector): A dynamic array that can grow
// or shrink in size. Vectors are widely used
// and provide efficient random access and fast iteration.
fn vector_example() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("vector numbers: {:?}", numbers);

    // Iterate through all of elements
    let mut sum = 0;
    for n in &numbers {
        sum += n;
    }
    println!("The sum of all vector numbers is: {}", sum);

    // When you want to iterate a part of collection you need to
    // Create slices
    sum = 0;
    for n in &numbers[1..3] {
        sum += n;
    }
    println!("The sum of vector numbers [2,3,4] is: {}", sum);

    // Map Iterator
    let squares: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("The (n-1) squared of vector are: {:?}", squares);

    // Filter Iterator
    let even_numbers: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("The even numbers of vector are: {:?}", even_numbers);
}

// HashMap: A hash table that stores key-value pairs.
// It allows you to quickly access values based on their keys.
fn hashmap_example() {
    let mut scores = HashMap::new();
    scores.insert("ali", 42);
    scores.insert("reza", 53);
    println!("hashmap scores: {:?}", scores);
    println!("hashmap scores keys: {:?}", scores.keys());
    println!("hashmap scores values: {:?}", scores.values());
}

// HashSet: Similar to HashMap, but it only contains
// unique values without any associated keys.
fn hashset_example() {
    let mut fruits = HashSet::new();
    fruits.insert("Apple");
    fruits.insert("Banana");
    fruits.insert("Grapes");
    fruits.insert("Banana");
    println!("hashset fruits: {:?}", fruits);
}


fn main() {
    vector_example();
    hashmap_example();
    hashset_example();
}
