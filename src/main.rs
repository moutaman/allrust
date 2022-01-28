// import stdin lib to handle user input from command line
use std::io::{stdin, stdout, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


fn handle(input: &mut String) {
    // handle function to deal with user input 
    // accepts mutable string variable named input
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("faild to handle");
}

fn main() {
    // create a mutiable variables to hold user inputs

    loop {
        let mut var1 = String::new();
        let mut var2 = String::new();
        let mut operator = String::new();

        println!("First Number: ");
        handle(&mut var1);

        println!("Second Number: ");
        handle(&mut var2);

        println!("Operation: [+-*/] ");
        handle(&mut operator);

        // Casting string vars to f32 type before executing math operations
        let _var1: f32 = var1.trim().parse().unwrap();
        let _var2: f32 = var2.trim().parse().unwrap();
        let operator: char = operator.trim().parse().unwrap();

        let operators = String::from("+-*/");

        if !operators.contains(operator){
            println!("Error, {} is not allowed operator", operator);
            continue;
        }

        let result = match operator {
            '+' => _var1 + _var2,
            '-' => _var1 - _var2,
            '*' => _var1 * _var2,
            '/' => _var1 / _var2,
            _ => panic!("error in operator")
        };

        println!("{} {} {} = {}", _var1, operator, var2, result);
    }
}
