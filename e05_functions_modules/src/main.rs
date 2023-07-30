// You can import different modules using the mod keyword
mod my_module;

// You can define an argument by reference using & or &mut values
fn add_numbers(a: i32, b: i32) -> i32 {
    let result = a + b;

    return result;
}

fn main() {
    println!("The sum of the numbers is {}", add_numbers(40, 60));

    // Call a function of my_module
    my_module::hello("amin");
}
