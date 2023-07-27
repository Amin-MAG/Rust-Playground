// What the heck is `!` characters? Why and when use
// macros when we writing code?

fn main() {
    // Scalar types
    let age: u32 = 30;
    let height: f64 = 5.9;
    let is_student: bool = true;
    let first_initial: char = 'J';

    // Compound types
    let tuple_example: (i32, f32, char) = (42, 3.14, 'R');
    let array_example: [i32; 5] = [1, 2, 3, 4, 5];
    let slice_example: &[i32] = &array_example[1..4];

    println!("Age: {}", age);
    println!("Height: {}", height);
    println!("Is student: {}", is_student);
    println!("First initial: {}", first_initial);

    println!("Tuple example: {:?}", tuple_example);
    println!("Array example: {:?}", array_example);
    println!("Slice example: {:?}", slice_example);
}