use std::io;

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
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                panic!("Lütfen 0 sayısı girmeyiniz.")
            }
        }
    }
}

fn main() {
    println!("Birinci sayıyı girin:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");
    let first_number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz bir sayı girdiniz!");
            return;
        }
    };

    println!("Yapılacak işlemi girin (add, subtract, multiply, divide):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");
    let operation = input.trim().to_lowercase();

    println!("İkinci sayıyı girin:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");
    let second_number: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz bir sayı girdiniz!");
            return;
        }
    };

    let op = match operation.as_str() {
        "add" => Operation::Add(first_number, second_number),
        "subtract" => Operation::Subtract(first_number, second_number),
        "multiply" => Operation::Multiply(first_number, second_number),
        "divide" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Geçersiz işlem seçtiniz!");
            return;
        }
    };

    let result = calculate(op);
    println!("Sonuç: {}", result);
}
