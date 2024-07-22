use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();

    println!("Enter the operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation = input.trim().to_string();

    input.clear();

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Please enter a valid number");

    let operation_enum = match operation.as_str() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };

    let result = calculate(operation_enum);

    println!("The result is: {}", result);
}

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}