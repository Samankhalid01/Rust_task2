use std::io::{self, Write};

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut input = String::new();

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let first_number: f64 = input.trim().parse().unwrap();

    input.clear();
    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let operation: char = input.trim().chars().next().unwrap();

    input.clear();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let second_number: f64 = input.trim().parse().unwrap();

    let operation_enum = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("Invalid operation"),
    };

    let result = calculate(operation_enum);

    println!("Result: {}", result);
}
