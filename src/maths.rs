// bring the standard library trait FromStr into scope
use std::str::FromStr;

// brings in the std::env module, which provides several
// useful functions and types for interacting with the execution environment, including
// the args function, which gives us access to the program’s command-line arguments.
use std::env;

// include code in src/maths/sum.rs
mod sum;

// entry point
pub fn run() {
    // immutable string. Can be able to be modified or changed.
    let operations: Vec<String> = env::args().collect();
    let operation = &operations[1].as_str();

    // parse args
    let numbers = parse_args();

    if numbers.len() > 0 {
        calculate(operation, &numbers);
    } else {
        println!("Usage: cargo run <operation> NUMBER ...");
        std::process::exit(1);
    }
}

// calculate based on operation
fn calculate(operation: &str, list: &Vec<i64>) {
    // pattern matching via the match keyword, which can be used like a C switch
    match operation {
        "sum" => {
            let answer = sum::run(&list);
            println!("The sum of {:?} is {}", list, answer);
        }
        _ => {
            println!("Required operations: 'sum' ...");
            println!("Usage cargo run <operation> Number ...");
            std::process::exit(1);
        }
    }
}

// Parse command line arguments
fn parse_args() -> Vec<i64> {
    let mut list = Vec::new();

    for arg in env::args().skip(2) {
        // attempt to parse our command-line argument arg as
        // an unsigned 64-bit integer.
        match i64::from_str(&arg) {
            Ok(num) => list.push(num),
            Err(_) => println!("Only integers are computed."),
        };
    }

    list
}
