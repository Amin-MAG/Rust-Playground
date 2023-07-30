use std::fs;
use std::io::Error;
use std::path::Path;

// Define an alias for the mock functions module
use mock_functions as mf;

mod mock_functions;

// The Result enum is a built-in type in Rust and is
// commonly used for functions that may return an error.
// It has two variants: Ok(T) to represent a successful result
// with a value of type T, and Err(E) to represent
// an error with a value of type E.
fn use_result_enum() {
    // An example without error
    let allowed_answer = mf::divide(15, 7);
    match allowed_answer {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {}", err),
    }

    // An example with error
    let un_allowed_answer = mf::divide(32, 0);
    match un_allowed_answer {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {}", err),
    }
}

// The Option enum is another built-in type in Rust,
// used for functions that may not always return a value.
// It has two variants: Some(T) to represent a successful
// result with a value of type T, and None to represent
// an absence of a value.
fn use_option_enum() {
    // An example that has an answer
    let numbers = vec![1, 3, 5, 7, 9];
    let target = 5;
    match mf::find_element(&numbers, target) {
        Some(index) => println!("The target has been found on {}", index),
        None => println!("Can not find anything!"),
    }


    // An example in which the answer is None
    let numbers = vec![1, 3, 0, 7, 9];
    let target = 5;
    match mf::find_element(&numbers, target) {
        Some(index) => println!("The target has been found on {}", index),
        None => println!("Can not find anything!"),
    }
}

// Create a custom error enum
enum MyError {
    FileNotFound,
    IOError(Error),
}

// A function that use a custom type of error
fn use_custom_error(file_path: &str) -> Result<String, MyError> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(MyError::FileNotFound);
    }

    fs::read_to_string(file_path).map_err(MyError::IOError)
}


// Error propagation
fn use_error_propagation() -> Result<i32, String> {
    // An example without error
    let allowed_answer = mf::calculate(15, 7)?;

    // An example with error
    let un_allowed_answer = mf::calculate(32, 0)?;

    if un_allowed_answer > allowed_answer {
        Ok(un_allowed_answer)
    } else {
        Ok(allowed_answer)
    }
}

fn main() -> Result<(), String> {
    use_result_enum();
    use_option_enum();
    let res = use_error_propagation()?;
    println!("message from error propagation function: {}", res);

    Ok(())
}
