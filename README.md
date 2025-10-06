# 🧮 Rust Calculator

A beautiful, interactive command-line calculator built with Rust. Features colorful output, input validation, and support for basic arithmetic operations.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## ✨ Features

- **Interactive CLI Interface** - User-friendly prompts guide you through each calculation
- **Colorful Output** - Color-coded interface for better readability
- **Input Validation** - Robust error handling ensures only valid numbers and operations are accepted
- **Multiple Operations** - Supports addition, subtraction, multiplication, and division
- **Division by Zero Protection** - Gracefully handles division by zero errors
- **Session Continuity** - Perform multiple calculations without restarting the program
- **Cross-Platform** - Works on Windows, macOS, and Linux

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher (install from [rustup.rs](https://rustup.rs/))

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-calculator.git
cd rust-calculator
```

2. Build the project:
```bash
cargo build --release
```

3. Run the calculator:
```bash
cargo run --release
```

## 📖 Usage

When you start the calculator, you'll be prompted to:

1. Enter your first number
2. Enter your second number
3. Choose an operation
4. View the result
5. Decide whether to perform another calculation

### Supported Operations

| Operation      | Input Options          |
|----------------|------------------------|
| Addition       | `add`, `+`, `a`       |
| Subtraction    | `subtract`, `-`, `s`  |
| Multiplication | `multiply`, `*`, `m`  |
| Division       | `divide`, `/`, `d`    |

### Example Session

```
Welcome to Calculator!

Enter first number: 15

Enter second number: 3

Enter operation (add, subtract, multiply, divide): multiply

15 * 3 = 45

Do you want to perform another calculation? (yes/no): yes

Enter first number: 100

Enter second number: 4

Enter operation (add, subtract, multiply, divide): /

100 / 4 = 25

Do you want to perform another calculation? (yes/no): no

Goodbye!
```

## 🛠️ Dependencies

This project uses the following crates:

- [`clearscreen`](https://crates.io/crates/clearscreen) - Cross-platform screen clearing
- [`colored`](https://crates.io/crates/colored) - Terminal text coloring

Add these to your `Cargo.toml`:

```toml
[dependencies]
clearscreen = "2.0"
colored = "2.0"
```

## 🏗️ Project Structure

```
calculator/
├── src/
│   └── main.rs          # Main calculator implementation
├── Cargo.toml           # Project dependencies
├── Cargo.lock           # Dependency lock file
└── README.md            # This file
```

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Inspired by the need for a simple, elegant calculator with great UX

---

**Happy Calculating!** 🎉