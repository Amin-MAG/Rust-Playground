// Defining a User struct
// The next line allow you to print your struct
#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: i8,
}

fn struct_example() {
    // To create a new instance of User struct
    let person1 = User {
        first_name: "Amin".to_string(),
        last_name: "MAG".to_string(),
        age: 24,
    };

    // Using and printing the struct information
    println!("The first name is {}, the last name is {}, and the age is {}",
             person1.first_name,
             person1.last_name,
             person1.age,
    );
    println!("The struct is equal to {:?}", person1);
}

// Enums allow you to define a type that can have
// multiple variants. Each variant can have different
// data associated with it. Enums are useful when you
// have a type that can represent different states or options.
enum Message {
    Text(String),
    Numeric(i32),
    Quit,
}

fn process_massage(msg: Message) {
    match msg {
        Message::Text(text) => {
            println!("Processing the text message: {}", text);
        }
        Message::Numeric(num) => {
            println!("Printing the number message: {}", num)
        }
        Message::Quit => {
            println!("Quitting");
        }
    }
}

// An example of creating and using enum
fn enum_example() {
    process_massage(Message::Text("This is a text message".to_string()));
    process_massage(Message::Numeric(50));
    process_massage(Message::Quit);
}

fn main() {
    struct_example();
    enum_example();
}
