use colored::*;
use clearscreen::clear;
use std::io::{self, Write};

/// Clears the terminal screen for a cleaner user interface.
/// 
/// Uses the clearscreen crate to provide cross-platform screen clearing
/// that works on Windows, macOS, and Linux.
/// 
/// # Panics
/// Panics if the screen cannot be cleared (rare, but possible if terminal
/// doesn't support clearing or has permission issues).
fn clear_screen() {
    clear().expect("Failed to clear screen");
}

/// Prompts the user to enter a number and validates the input.
/// 
/// Continuously prompts until a valid floating-point number is provided.
/// 
/// # Arguments
/// * `prompt` - The message to display to the user
/// 
/// # Returns
/// A valid f64 number entered by the user
fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt.cyan());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("{}", "\nThat's not a valid number. Try again!".red()),
        }
    }
}

/// Prompts the user to select a mathematical operation.
/// 
/// Validates that the input is one of: add, subtract, multiply, or divide.
/// The input is case-insensitive.
/// 
/// # Arguments
/// * `prompt` - The message to display to the user
/// 
/// # Returns
/// A lowercase string representing the chosen operation
fn read_choice(prompt: &str) -> String {
    loop {
        print!("{}", prompt.cyan());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "add" | "subtract" | "multiply" | "divide" => return input,
            _ => println!("{}", "Invalid operation! Please type one of: add, subtract, multiply, divide".red()),
        }
    }
}

/// Performs a mathematical operation on two numbers.
/// 
/// # Arguments
/// * `a` - The first operand
/// * `b` - The second operand
/// * `choice` - The operation to perform ("add", "subtract", "multiply", or "divide")
/// 
/// # Returns
/// * `Some(f64)` - The result of the calculation if successful
/// * `None` - If the operation cannot be performed (e.g., division by zero)
fn calculate(a: f64, b: f64, choice: &str) -> Option<f64> {
    match choice {
        "add" => Some(a + b),
        "subtract" => Some(a - b),
        "multiply" => Some(a * b),
        "divide" => {
            if b == 0.0 {
                println!("{}", "\nError: Division by zero! Let's try again.".red().bold());
                None
            } else {
                Some(a / b)
            }
        }
        _ => {
            println!("{}", "Invalid operation".red());
            None
        }
    }
}

/// Prompts the user to decide whether to perform another calculation.
/// 
/// Accepts "yes"/"y" or "no"/"n" (case-insensitive).
/// 
/// # Arguments
/// * `prompt` - The message to display to the user
/// 
/// # Returns
/// * `true` - If the user wants to continue
/// * `false` - If the user wants to exit
fn ask_again(prompt: &str) -> bool {
    loop {
        print!("{}", prompt.yellow());
        io::stdout()
            .flush()
            .expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("{}", "Please type yes or no.".red()),
        }
    }
}

/// Main entry point for the calculator application.
/// 
/// Provides an interactive command-line calculator that:
/// - Accepts two numbers from the user
/// - Performs basic arithmetic operations (add, subtract, multiply, divide)
/// - Displays the result with colored output
/// - Allows multiple calculations in a single session
fn main() {
    println!("{}", "Welcome to Calculator!".bright_blue().bold());
    
    loop {
        // Keep asking for valid inputs and calculation until we get a result
        let result = loop {
            let a = read_number("\nEnter first number: ");
            let b = read_number("\nEnter second number: ");
            let choice = read_choice("\nEnter operation (add, subtract, multiply, divide): ");

            // Attempt calculation; retry if it fails (e.g., division by zero)
            if let Some(res) = calculate(a, b, &choice) {
                break res;
            }
        };

        clear_screen(); // Clear the screen before showing the result
        println!("\n{} {}\n", "Result:".green().bold(), result.to_string().bright_green());

        // Exit if user doesn't want another calculation
        if !ask_again("Do you want to perform another calculation? (yes/no): ") {
            println!("{}", "\nGoodbye!\n".bright_blue());
            break;
        } else {
            clear_screen(); // Clear the screen for the next calculation
        }
    }
}