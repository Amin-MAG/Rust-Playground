// Ownership
// In Rust, every value has an owner, and a value
// can only have one owner at a time. When the owner
// goes out of scope, Rust automatically deallocates
// the memory associated with the value. This automatic
// memory management without the need for a garbage collector
// or manual memory management is one of Rust's strengths.
fn ownership_example() {
    // The "hello" string literal has an owner which is 'greeting'.
    let greeting = "hello";
    // When 'hello' goes out of scope (here), its memory is automatically deallocated.

    println!("The greeting variable is the owner of string: {}", greeting);
}

// Borrowing
// To allow multiple parts of the code to access
// a value without transferring ownership, Rust
// uses borrowing. Borrowing allows you to create
// references to values, and multiple references
// can coexist, but they have certain restrictions
// to prevent data races and other memory safety issues.
//
// Mutable Borrow (&mut T): Allows a reference to modify the value.
// Immutable Borrow (&T): Allows a reference to read the value but not modify it.
fn borrowing_example() {
    // 'x' has the ownership of integer '5' in the memory.
    let mut x = 5;

    // Create a mutable borrow - y is a mutable reference to 'x'
    let y = &mut x;
    *y *= 2;

    // Create an immutable borrow - z is an immutable reference to 'x'
    let z = &x;

    // Create an immutable borrow for the second time
    let w = &x;

    // Uncommenting the line below will cause a compile-time error
    // because we can't have both a mutable and immutable reference to 'x' in the same scope.
    // println!("The value of y which is an immutable reference of x: {}", *y);
    println!("The value of z which is an immutable reference of x: {}", *z);
    println!("The value of w which is an immutable reference of x: {}", *w);
}

fn main() {
    ownership_example();
    borrowing_example();
}
