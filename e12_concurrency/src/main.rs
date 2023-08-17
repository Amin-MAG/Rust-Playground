use std::io::{Error, ErrorKind};
use std::thread;
use std::time::Duration;

// Concurrency in Rust refers to the ability to
// perform multiple tasks or operations simultaneously.
// Rust provides powerful tools to work with concurrency,
// including support for creating threads and asynchronous
// programming using async/await.
//
// In Rust, you can create threads using the
// std::thread module. Threads allow you to run
// multiple blocks of code concurrently, enabling
// parallel execution of tasks.
fn thread_example() {
    // Use spawn to create a new thread
    let handle = thread::spawn(|| {
        // Code to be executed in the new thread is passed with
        // an anonymous function
        for i in 1..=5 {
            thread::sleep(Duration::from_secs(1));
            println!("Thread: {}", i);
        }
    });

    // Code in the main thread
    for i in 1..=5 {
        thread::sleep(Duration::from_millis(500));
        println!("Main: {}", i);
    }

    // Wait for the new thread to finish its execution
    handle.join().unwrap();
}

// Asynchronous programming in Rust enables you to perform
// non-blocking I/O operations without blocking the main
// thread. It's particularly useful for handling
// I/O-bound tasks efficiently, such as reading/writing
// files or making network requests.
//
// Asynchronous programming in Rust is achieved using
// the async and await keywords. These keywords allow
// you to define asynchronous functions and await the
// completion of asynchronous operations.
async fn read_file_contents(file_path: &str) -> Result<String, Error> {
    thread::sleep(Duration::from_millis(3000));
    if file_path == "example.txt" {
        Ok("The content of the example.txt".to_string())
    } else {
        Err(Error::new(ErrorKind::NotFound, "can not find this file".to_string()))
    }
}

async fn async_example() {
    let file_path = "example.txt";
    // Start the async function and continue to run this function.
    let res = read_file_contents(file_path);
    thread::sleep(Duration::from_millis(1000));
    println!("the file path includes {} characters: ", file_path.len());
    // Here we need the answer so we wait.
    match res.await {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }
    println!("the end of async Example")
}

#[tokio::main]
async fn main() {
    thread_example();
    async_example().await;
}