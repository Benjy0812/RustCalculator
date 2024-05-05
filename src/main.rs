#![allow(unused)]//disables unused warnings
//src\main.rs
//modules
use std::{io, string};//adds the io and string modules to the program
//start of main function
fn main() {//main function
    loop{//loop the program
        //Ask the user to enter the first number
        println!("Enter a number:");//print the string
        let mut num1_str: String = String::new();//create a new string with the name num1_str
        io::stdin().read_line(&mut num1_str).expect("Failed to read line");//read user input and store it in num1_str
        let num1: f64 = num1_str.trim().parse().expect("Please type a number!");//assign num1 to num1_str
        // Ask the user to enter the second number
        println!("Enter another number:");//print the string
        let mut num2_str: String = String::new();//create a new string with the name num2_str
        io::stdin().read_line(&mut num2_str).expect("Failed to read line");//read user input and store it in num2_str
        let num2: f64 = num2_str.trim().parse().expect("Please type a number!");//assign num2 to num2_str
        // Ask the user to select an operator
        println!("Enter the operator (+, -, *, /):");//print the string
        let mut operator_str: String = String::new();//create a new string with the name operator_str
        io::stdin().read_line(&mut operator_str).expect("Failed to read line");//read user input and store it in operator_str
        let operator: &str = operator_str.trim();//assign operator_str to operator
        // Perform the calculation
        let result = String::new();//create a new string with the name result
        let result: f64 = match operator {//match operator to the result
            "+" => num1 + num2,//if operator = + then return num1 + num2
            "-" => num1 - num2,//if operator = - then return num1 - num2
            "*" => num1 * num2,//if operator = * then return num1 * num2
            "/" => num1 / num2,//if operator = / then return num1 / num2
            _ => panic!("Invalid operator"),//if operator is not + - * or / then panic
        };//end of match
        println!("Result: {}", result);//print the result
        // Ask the user if they want to continue
        println!("Do you want to continue? (y/n)");
        let mut continue_input: String = String::new();//create a new string with the name continue_input
        io::stdin().read_line(&mut continue_input).expect("Failed to read line");//read user input and store it in continue_input
        let continue_choice: &str = continue_input.trim();//assign continue_input to continue_choice
        // Check if the user wants to continue
        if continue_choice.to_lowercase().starts_with('n') {//check if continue_choice starts with 'n'
            break; // Exit the loop if the user chooses not to continue
        }//end of if continue_choice
    }//end of loop
}//end of main function
