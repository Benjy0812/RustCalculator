use std::io::{self, Write};

fn calculate(a: f64, b: f64, choice: &str) -> f64 {
    match choice {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => {
            if b == 0.0 {
                println!("Error: Division by zero");
                0.0
            } else {
                a / b
            }
        }
        _ => {
            println!("Invalid operation");
            0.0
        }
    }
}

// Helper to read a number
fn read_number(prompt: &str) -> f64 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

// Helper to read operation
fn read_choice(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let a = read_number("\nEnter first number: ");
    let b = read_number("\nEnter second number: ");
    let choice = read_choice("\nEnter operation (add, subtract, multiply, divide): ");

    let result = calculate(a, b, &choice);
    println!("\nResult: {}\n", result);
}
