use clearscreen::clear;
use colored::*;
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
/// Invalid inputs display an error message and re-prompt the user.
///
/// # Arguments
/// * `prompt` - The message to display to the user
///
/// # Returns
/// A valid f64 number entered by the user
///
/// # Panics
/// Panics if stdout cannot be flushed or stdin cannot be read.
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
/// Validates that the input is one of the supported operations.
/// The input is case-insensitive and accepts:
/// - Full words: "add", "subtract", "multiply", "divide"
/// - Symbols: "+", "-", "*", "/"
/// - Single letters: "a", "s", "m", "d"
///
/// # Arguments
/// * `prompt` - The message to display to the user
///
/// # Returns
/// A string containing the operation symbol ("+", "-", "*", or "/")
///
/// # Panics
/// Panics if stdout cannot be flushed or stdin cannot be read.
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
            "add" | "+" | "a" => return "+".to_string(),
            "subtract" | "-" | "s" => return "-".to_string(),
            "multiply" | "*" | "m" => return "*".to_string(),
            "divide" | "/" | "d" => return "/".to_string(),
            _ => println!("{}", "Invalid operation! Please type one of: add, subtract, multiply, divide (or +, -, *, /)".red()),
        }
    }
}

/// Performs a mathematical operation on two numbers.
///
/// # Arguments
/// * `a` - The first operand
/// * `b` - The second operand
/// * `choice` - The operation symbol ("+", "-", "*", or "/")
///
/// # Returns
/// * `Some(f64)` - The result of the calculation if successful
/// * `None` - If the operation cannot be performed (e.g., division by zero)
///
/// # Panics
/// Panics if an invalid operation symbol is passed (this should never happen
/// as `read_choice` only returns valid operators).
fn calculate(a: f64, b: f64, choice: &str) -> Option<f64> {
    match choice {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),
        "/" => {
            if b.abs() < f64::EPSILON {
                println!(
                    "{}",
                    "\nError: Division by zero! Let's try again.".red().bold()
                );
                None
            } else {
                Some(a / b)
            }
        }
        _ => unreachable!("read_choice should only return valid operators"),
    }
}

/// Prompts the user to decide whether to perform another calculation.
///
/// Accepts "yes"/"y" or "no"/"n" (case-insensitive).
/// Invalid inputs display an error message and re-prompt the user.
///
/// # Arguments
/// * `prompt` - The message to display to the user
///
/// # Returns
/// * `true` - If the user wants to continue
/// * `false` - If the user wants to exit
///
/// # Panics
/// Panics if stdout cannot be flushed or stdin cannot be read.
fn ask_again(prompt: &str) -> bool {
    loop {
        print!("{}", prompt.yellow());
        io::stdout().flush().expect("Failed to flush stdout");

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
/// Provides an interactive command-line calculator with the following features:
/// - Accepts two floating-point numbers from the user
/// - Performs basic arithmetic operations: addition, subtraction, multiplication, division
/// - Displays results with colored output for better readability
/// - Handles division by zero gracefully with error messages
/// - Clears the screen between calculations for a clean interface
/// - Allows multiple calculations in a single session
/// - Supports flexible operation input (words, symbols, or shortcuts)
///
/// The program continues running until the user chooses to exit.
fn main() {
    println!("{}", "Welcome to Calculator!".bright_blue().bold());

    loop {
        // Keep asking for valid inputs and calculation until we get a result
        let (a, b, choice, result) = loop {
            let a = read_number("\nEnter first number: ");
            let choice = read_choice("\nEnter operation (+, -, *, / or add, subtract, multiply, divide): ");
            let b = read_number("\nEnter second number: ");

            // Attempt calculation; retry if it fails (e.g., division by zero)
            if let Some(result) = calculate(a, b, &choice) {
                break (a, b, choice, result);
            }
        };

        clear_screen();
        println!("\n{} {} {} = {}\n", 
            a.to_string().bright_blue().bold(),
            choice.bright_yellow().bold(),
            b.to_string().bright_blue().bold(),
            result.to_string().bright_green().bold()
        );
        // Exit if user doesn't want another calculation
        if !ask_again("Do you want to perform another calculation? (yes/no): ") {
            println!("{}", "\nGoodbye!\n".bright_blue());
            break;
        } else {
            clear_screen(); // Clear the screen for the next calculation
        }
    }
}