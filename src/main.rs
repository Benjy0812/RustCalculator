#![allow(unused)]//disables unused warnings
//src\main.rs
//modules
use std::io; // Import the io module for user input

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
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

fn main() {
    loop {
        // Read the first and second numbers
        let num1 = read_number("Enter the first number:");
        let num2 = read_number("Enter the second number:");

        // Ask the user to select an operator
        println!("Enter the operator (+, -, *, /):");
        let mut operator = String::new();
        if io::stdin().read_line(&mut operator).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }
        let operator = operator.trim();

        // Perform the calculation
        let result = match operator {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid operator. Please use one of +, -, *, /.");
                continue;
            }
        };

        // Display the result
        println!("Result: {}", result);

        // Ask the user if they want to continue
        println!("Do you want to continue? (y/n):");
        let mut continue_input = String::new();
        if io::stdin().read_line(&mut continue_input).is_err() {
            println!("Error: Failed to read input.");
            continue;
        }
        if continue_input.trim().eq_ignore_ascii_case("n") {
            break;
        }
    }
}
