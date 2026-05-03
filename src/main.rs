use core::panic;
use std::{env::args, fs, io::Read};

use antlr_rust::tree::ParseTreeVisitorCompat;

use imp_interpreter::interpreter::ImpInterpreter;

fn main() {
    let args: Vec<String> = args().collect();

    let mut input_string = String::new();
    match args.get(1) {
        Some(input_path) => {
            let mut file = fs::File::open(input_path).unwrap_or_else(|e| panic!("{e}"));

            file.read_to_string(&mut input_string)
                .unwrap_or_else(|e| panic!("{e}"));
        }
        None => {
            println!("Enter a string to parse:");

            // Get user input from stdin
            std::io::stdin()
                .read_line(&mut input_string)
                .unwrap_or_else(|e| panic!("{e}"));
        }
    }

    let mut interpreter = ImpInterpreter::new();
    let tree = ImpInterpreter::parse(&input_string);
    let intperpreted_result = interpreter.visit(&*tree);

    println!("{intperpreted_result}");
    println!("\n{:#?}", interpreter);
}
