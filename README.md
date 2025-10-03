# Rust CLI Calculator

A simple command-line calculator written in Rust that performs basic arithmetic operations: addition, subtraction, multiplication, and division.

## Features

- Performs a single calculation per run
- Supports the following operations:
  - `add` – addition
  - `subtract` – subtraction
  - `multiply` – multiplication
  - `divide` – division
- Handles division by zero gracefully
- Uses `f64` for higher numeric precision

## Usage

1. Clone the repository or download the code.
2. Build and run using `cargo`:

   ```bash
   cargo run
   ```

3. Enter the first number, second number, and operation when prompted:

   ```text
   Enter first number: 10

   Enter second number: 5

   Enter operation (add, subtract, multiply, divide): divide
   ```

4. The result will be displayed:

   ```text
   Result: 2
   ```

## Notes

- Currently, the program performs **only one calculation per execution**.
- A loop for continuous calculations will be **added in a future update**.

## Example

```text
Enter first number: 12

Enter second number: 4

Enter operation (add, subtract, multiply, divide): multiply

Result: 48
```
