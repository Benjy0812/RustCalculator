use std::io::{self, Write}; // import io and flush for nicer prompts

// Reads a number from the user
fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().unwrap(); // ensures prompt appears before input

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Error: Please enter a valid number!"),
        }
    }
}

// Reads an operator from the user
fn read_operator() -> String {
    loop {
        print!("Enter the operator (+, -, *, /): ");
        io::stdout().flush().unwrap();

        let mut operator = String::new();
        if io::stdin().read_line(&mut operator).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }

        let op = operator.trim();
        if ["+", "-", "*", "/"].contains(&op) {
            return op.to_string();
        } else {
            println!("Invalid operator. Please use one of +, -, *, /.");
        }
    }
}

// Performs the calculation
fn calculate(num1: f64, num2: f64, operator: &str) -> Option<f64> {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => None, // shouldn't happen because we validate operator
    }
}

fn main() {
    loop {
        let num1 = read_number("Enter the first number:");
        let num2 = read_number("Enter the second number:");
        let operator = read_operator();

        if let Some(result) = calculate(num1, num2, &operator) {
            println!("======================");
            println!("Result: {}", result);
            println!("======================");
        }

        print!("Do you want to continue? (yes/no): ");
        io::stdout().flush().unwrap();

        let mut continue_input = String::new();
        if io::stdin().read_line(&mut continue_input).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }

        if continue_input.trim().eq_ignore_ascii_case("no") {
            println!("Goodbye!");
            break;
        }
    }
}
