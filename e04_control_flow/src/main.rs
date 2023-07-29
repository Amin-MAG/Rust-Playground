use std::io;

fn condition_examples() {
    // Read user input from STDOUT
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("can not read the user input from STDIN");

    // Calculate the split string
    let split: Vec<&str> = user_input.trim().split(' ').collect();
    println!("{:?}", split);

    // Parse the string and convert it to integer
    let age_number = split[1].parse::<u32>();
    match age_number {
        Ok(age) => {
            println!("Parsed integer: {}", age);   

            // Control Flow 0
            if age > 17 {
                println!("the age is ok");
            } else {
                println!("the person is under age");
            }
        }
        Err(err) => {
            println!("Error parsing integer: {:?}", err);
        }
    }
    
    // Control Flow 1
    let name = split[0];
    let family_name;
    match name {
        "ali" => {
            family_name = "mohammadi";
        }
        "mohsen" => {
            family_name = "ahmadi";
        }
        "amin" => {
            family_name = "rezae";
        }
        "mohammad" | "mohamad" => {
            family_name = "kasra";
        }
        _ => { 
            family_name = "none!";
        }
    }

    println!("{} {}", name, family_name);
}

fn loop_examples() {
    // Use loop iteration
    let mut count = 0;
    loop { 
        if count == 5 { 
            break
        }
        println!("Counting number {}", count);
        count += 1;
    }

    // Use while iteration
    let mut number = 0;
    while number < 5 {
        println!("Number is: {}", number);
        number += 1;
    }

    // Use for iteration
    for i in 1..=5 {
        println!("For iteration: {}", i);
    }

    // Iterate through a vector
    let mut test_vector = vec![1, 2, 3, 4, 5];
    for num in &mut test_vector {
        *num = *num*2;
        println!("The new number in the vector is {}", *num);
    }

    // Iterate through a vector with a iterator
    for (idx, num) in test_vector.iter().enumerate() {
        println!("The index is {} and the value is {}", idx, num)
    }
}

fn main() {
    condition_examples();
    loop_examples();
}