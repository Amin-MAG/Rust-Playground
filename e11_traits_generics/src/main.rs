use std::fmt::Display;

trait Printable {
    fn print(&self);
}


#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Printable for User {
    fn print(&self) {
        println!("My name is {} {}. I am {} years old.",
                 self.first_name,
                 self.last_name,
                 self.age,
        )
    }
}


#[derive(Debug)]
struct Letter {
    title: String,
    content: String,
}

impl Printable for Letter {
    fn print(&self) {
        println!("The title of the letter is {}. Here is the content: {}",
                 self.title,
                 self.content,
        )
    }
}

fn print_if_printable<T: Printable>(item: &T) {
    item.print();
}

fn trait_example() {
    let user = User {
        first_name: "Amin".to_string(),
        last_name: "MAG".to_string(),
        age: 24,
    };

    let letter = Letter {
        title: "Housing Fee".to_string(),
        content: "The amount of money you should pay is $152.32.".to_string(),
    };

    print_if_printable(&user);
    print_if_printable(&letter);
}


// Define the Container trait
trait Container {
    type Item: Display;
    fn get(&self) -> &Self::Item;

    // Default implementation for the 'get' method
    fn print(&self) {
        let n = self.get();
        println!("Container item: {}", n);
    }
}

// Implement the Container trait for a custom type MyContainer
// We can set some rules for the type of T to be Display.
struct MyContainer<T> where T: Display {
    item: T,
}

// Here we implement the necessary function of the trait.
impl<T> Container for MyContainer<T> where T: Display {
    type Item = T;

    fn get(&self) -> &T {
        &self.item
    }
}


fn generics_example() {
    // Create an instance of MyContainer holding an i32
    let my_container = MyContainer { item: 42 };

    // Using the default implementation of the 'print' method
    my_container.print();

    // Access the item directly using the 'get' method
    let item = my_container.get();
    println!("Item: {}", item);
}


fn main() {
    trait_example();
    generics_example();
}