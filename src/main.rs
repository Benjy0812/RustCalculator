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
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<f64>() { // try to parse input as f64
            Ok(num) => return num, // if valid number return it
            Err(_) => println!("\nThat's not a valid number. Try again!"), // if not a valid number prompt again
        }
    }
}

// Helper to read operation
fn read_choice(prompt: &str) -> String {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_lowercase();

        match input.as_str() {
            "add" | "subtract" | "multiply" | "divide" => return input,
            _ => println!("Invalid operation! Please type one of: add, subtract, multiply, divide"),
        }
    }
}

fn ask_again(prompt: &str) -> bool {
    loop {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "yes" | "y" => return true,
            "no" | "n" => return false,
            _ => println!("Please type yes or no."),
        }
    }
}


fn main() {
    loop {
        let a = read_number("\nEnter first number: ");
        let b = read_number("\nEnter second number: ");
        let choice = read_choice("\nEnter operation (add, subtract, multiply, divide): ");

        let result = calculate(a, b, &choice);
        println!("\nResult: {}\n", result);
        
        let again = ask_again("Do you want to perform another calculation? (yes/no): ");
        if !again {
            println!("\nGoodbye!\n");
            break;
        }
    }
}
