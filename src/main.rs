use std::io::{self, Write};

fn main() {
    println!("Hello, world!");

    // Main Loop
    loop {
        // Prompt the user for the first number
        println!("Please Choose an item from the Following Menu.");
        println!("Menu:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Raise to a power");
        println!("6. Square Root");
        println!("7. Quit");

        // Prompt the user for their choice
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        // Call functions that coorespond to the menu
        match choice {
            1 => add(),
            2 => sub(),
            3 => multiply(),
            4 => divide(),
            5 => raise_to_pow(),
            6 => sqaure_root(),
            7 => break,
            _ => {
                println!("Invalid choice. Please enter a valid menu option.");
                continue;
            }
        };

    }

}

// Function that returns two numbers
fn get_numbers() -> (f64, f64)
{
    let mut input = String::new();

    // Prompt the user for the first number
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = input.trim().parse().unwrap();

    // Prompt the user for the second number
    input.clear();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = input.trim().parse().unwrap();

    (num1, num2)
}

// Function that returns one number
fn get_number() -> f64
{
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num: f64 = input.trim().parse().unwrap();

    num
}

// Add function
fn add() {
    // Get numbers
    let (num1, num2) = get_numbers();

    // Perform addition, display result
    let result = num1 + num2;
    println!("Result: {}", result);
}

// Subtract function
fn sub() {
    // Get numbers
    let (num1, num2) = get_numbers();

    // Perform subtraction, display result
    let result = num1 - num2;
    println!("Result: {}", result);
}

// Multiplication function
fn multiply() {
    // Get numbers
    let (num1, num2) = get_numbers();

    // Perform multiplication, display result
    let result = num1 * num2;
    println!("Result: {}", result);
}

// Division function
fn divide() {
    // Get numbers
    let (num1, num2) = get_numbers();

    // Perform division, display result
    let result = num1 / num2;
    println!("Result: {}", result);
}

// Raise to a power function
fn raise_to_pow() {
    // Get numbers
    let (num1, num2) = get_numbers();

    // Raise number to exponent, display result
    let result = num1.powf(num2);
    println!("Result: {}", result);
}

// Square root function
fn sqaure_root() {
    // Get a number
    let num = get_number();

    // Square root it, display result
    let result = num.sqrt();
    println!("Result: {}", result);
}