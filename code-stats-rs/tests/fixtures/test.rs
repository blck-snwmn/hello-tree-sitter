fn main() {
    println!("Hello, World!");
}

struct Calculator {
    value: i32,
}

impl Calculator {
    fn new() -> Self {
        Self { value: 0 }
    }
    
    fn add(&mut self, x: i32) -> i32 {
        self.value += x;
        self.value
    }
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

enum Operation {
    Add,
    Subtract,
    Multiply,
}

fn calculate(op: Operation, a: i32, b: i32) -> i32 {
    match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
    }
}