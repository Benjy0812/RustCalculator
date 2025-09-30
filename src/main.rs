use std::io::{self, Write}; 
// Import Rust's input/output library
// `Write` allows us to flush output, so prompts show immediately

// ============================================================================
// Function: read_number
// Purpose: Continuously ask for a number until the user enters a valid one
// ============================================================================
fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{} ", prompt);
        io::stdout().flush().unwrap(); // ensures prompt is displayed before input

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }

        match input.trim().parse() {
            Ok(num) => return num, // valid number, return it
            Err(_) => println!("Error: Please enter a valid number!"),
        }
    }
}

// ============================================================================
// Function: read_operator
// Purpose: Continuously ask for an operator until valid (+, -, *, /)
// ============================================================================
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

// ============================================================================
// Function: calculate
// Purpose: Perform math based on operator and return result
// ============================================================================
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
        _ => None, // should never happen
    }
}

// ============================================================================
// MAIN PROGRAM
// ============================================================================
fn main() {
    loop {
        let num1 = read_number("Enter the first number:");
        let num2 = read_number("Enter the second number:");
        let operator = read_operator();

        if let Some(result) = calculate(num1, num2, &operator) {
            println!("===================== Result =====================");
            println!("Result: {}", result);
            println!("==================================================");
        }

        print!("Do you want to continue? (yes/y or no/n): ");
        io::stdout().flush().unwrap();

        let mut continue_input = String::new();
        if io::stdin().read_line(&mut continue_input).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }

        let answer = continue_input.trim().to_lowercase();
        if answer == "n" || answer == "no" {
            println!("===================== Goodbye! ===================");
            break;
        }
    }
}
