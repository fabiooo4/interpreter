use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
};

use imp_interpreter::{
    interpreter::ImpInterpreter,
    parser::{implexer::ImpLexer, impparser::ImpParser},
};

fn main() {
    println!("Enter a string to parse:");

    // Get user input from stdin
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let input = InputStream::new(input_string.trim());

    // Create a TokenSource from the CharStream using the Imp grammar
    let lexer = ImpLexer::new(input);

    // Obtain the tokens from the TokenSource as a TokenStream
    let tokens = CommonTokenStream::new(lexer);

    // Create a parser that parses the Imp grammar
    let mut parser = ImpParser::new(tokens);

    // Execute the grammar from the 'main' nonterminal symbol
    let tree = parser.main().unwrap();

    let mut interpreter = ImpInterpreter(0.);
    let intperpreted_result = interpreter.visit(&*tree);

    println!("{}", tree.to_string_tree(&*parser));

    println!("Interpreted result = {intperpreted_result}");
}
