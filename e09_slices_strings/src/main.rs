fn slice_example() {
    // Slices
    // A slice is a reference to a contiguous portion of a collection,
    // such as an array, a vector, or a string. Slices are useful when
    // you need to work with only a part of the data without copying or
    // taking ownership of the entire collection.
    let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    // Create an slice from first until the forth element without copping
    let slice = &mut numbers[..4];
    println!("The first number in slice is {}", slice[0]);
    println!("The slice is {:?}", slice);

    // Change the value of a mutable borrowed variable of 'numbers'
    slice[0] = -1;
    println!("The numbers are {:?}", numbers);
    println!("The first number in numbers is {}", numbers[0]);
}

fn string_manipulation_examples() {
    // Concat strings
    let first_name = "Amin";
    let last_name = "MAG";
    let name = format!("{}-{}", first_name, last_name);
    println!("Combined string with size of {} is: {}", name.len(), name);

    // Split the full name to first_name and last_name
    let words: Vec<_> = name.split('-').collect();
    let first_name = words[0];
    let last_name = words[1];
    println!("By splitting the first_name is {} and the last name is {}", first_name, last_name);

    // Iterate through a string
    for (idx, c) in first_name.chars().enumerate() {
        // To cast a value to another type use 'as' keyword
        println!("Character with index {} is {}", idx, c as u32);
    }

    // Search for a substring
    let sentence = "Rust is a good language!";
    if sentence.contains("good") {
        println!("The sentence is positive!");
    } else {
        println!("There is no good word");
    }

    // Replace a substring
    let replaced = sentence.replace("Rust", "Go");
    println!("replaced string is {}", replaced);
}

fn main() {
    slice_example();
    string_manipulation_examples();
}
