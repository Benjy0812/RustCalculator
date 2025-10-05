use colored::*;
use std::io::{self, Write};

// Helper to read a number
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

// Helper to read operation
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

// Now returns Option<f64>
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

fn main() {
    println!("{}\n", "Welcome to Calculator!".bright_blue().bold());
    
    loop {
        let result = loop {
            let a = read_number("\nEnter first number: ");
            let b = read_number("\nEnter second number: ");
            let choice = read_choice("\nEnter operation (add, subtract, multiply, divide): ");

            if let Some(res) = calculate(a, b, &choice) {
                break res;
            }
        };

        println!("\n{} {}\n", "Result:".green().bold(), result.to_string().bright_green());

        if !ask_again("Do you want to perform another calculation? (yes/no): ") {
            println!("{}", "\nGoodbye!\n".bright_blue());
            break;
        }
    }
}