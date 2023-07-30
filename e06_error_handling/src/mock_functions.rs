// When you want to write a function can have some errors,
// You can use result.
pub fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        // Returning a message due to error
        return Err(String::from("Division by zero is not allowed."));
    }

    // Return a safe answer with OK
    return Ok(a / b);
}


// find_element
// If your function might have different kind of output
// based on the situation, you can use options
pub fn find_element(numbers: &[i32], target: i32) -> Option<usize> {
    for (idx, num) in numbers.iter().enumerate() {
        if *num == target {
            return Some(idx);
        }
    }

    return None;
}


// calculate function do the division and propagate
// the possible error
pub fn calculate(a: i32, b: i32) -> Result<i32, String> {
    let res = divide(a, b)?;
    return Ok(res);
}
