# Project Structure

```
int-expr-interpreter/
├── grammars/
│   ├── IntExpr.g4
│   ├── Nat.g4
│   └── VisitorBasic.g4
├── src/
│   ├── interpreter.rs
│   ├── lib.rs
│   ├── main.rs
│   └── parser.rs
├── Cargo.toml
├── README.md
└── build.rs
```

# Project Files

## File: `grammars/IntExpr.g4`

```g4
grammar IntExpr;
import Nat;

main : exp EOF;

// Labels begin with # and rename each node of the ParseTree
exp : nat                                  # val
    | LPAR left=exp ADD right=exp RPAR     # add
    | LPAR left=exp MUL right=exp RPAR     # mul
    ;

LPAR : '(' ;
RPAR : ')' ;
ADD  : '+' ;
MUL  : '*' ;

// This rule ignores all whitespace
WS   : [ \t\n\r]+ -> skip ;

```

## File: `grammars/Nat.g4`

```g4
grammar Nat;

main : nat EOF ;
nat  : '0' | '1' seq | '2' seq | '3' seq | '4' seq | '5' seq | '6' seq | '7' seq | '8' seq | '9' seq ;
seq  : | '0' seq | '1' seq | '2' seq | '3' seq | '4' seq | '5' seq | '6' seq | '7' seq | '8' seq | '9' seq ;

```

## File: `grammars/VisitorBasic.g4`

```g4
grammar VisitorBasic;

s: a EOF;

a: '1';

```

## File: `src/interpreter.rs`

```rust
use core::panic;

use antlr_rust::tree::{ErrorNode, ParseTree, ParseTreeVisitorCompat};

use crate::parser::{
    intexprparser::{
        AddContext, IntExprParserContextType, MainContext, MainContextAttrs, ValContext,
    },
    intexprvisitor::IntExprVisitorCompat,
    visitorbasicparser::SContextAttrs,
};

pub struct IntExprInterpreter(pub u32);

impl ParseTreeVisitorCompat<'_> for IntExprInterpreter {
    type Node = IntExprParserContextType;
    type Return = u32;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        panic!("Error encountered: {}", _node.symbol)
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + next
    }
}

impl IntExprVisitorCompat<'_> for IntExprInterpreter {
    fn visit_main(&mut self, ctx: &MainContext<'_>) -> Self::Return {
        self.visit(&*ctx.exp().unwrap())
    }

    fn visit_val(&mut self, ctx: &ValContext<'_>) -> Self::Return {
        ctx.get_text()
            .parse()
            .expect("Failed to parse integer value")
    }

    fn visit_add(&mut self, ctx: &AddContext<'_>) -> Self::Return {
        let left = self.visit(
            &*ctx
                .left
                .clone()
                .expect("Failed to parse left value for addition"),
        );
        let right = self.visit(
            &*ctx
                .right
                .clone()
                .expect("Failed to parse right value for addition"),
        );

        left + right
    }

    fn visit_mul(&mut self, ctx: &crate::parser::intexprparser::MulContext<'_>) -> Self::Return {
        let left = self.visit(
            &*ctx
                .left
                .clone()
                .expect("Failed to parse left value for multiplication"),
        );
        let right = self.visit(
            &*ctx
                .right
                .clone()
                .expect("Failed to parse right value for multiplication"),
        );

        left * right
    }
}

pub struct TestVisitor(pub u32);
impl ParseTreeVisitorCompat<'_> for TestVisitor {
    type Node = crate::parser::visitorbasicparser::VisitorBasicParserContextType;
    type Return = u32;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }

    fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
        // format!("Error encountered: {}", _node.symbol)

        panic!("Error encountered: {}", _node.symbol);
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {

        println!("Aggregate: {aggregate}, Next: {next}");
        aggregate + next
    }
}
impl crate::parser::visitorbasicvisitor::VisitorBasicVisitorCompat<'_> for TestVisitor {
    fn visit_s(&mut self, ctx: &crate::parser::visitorbasicparser::SContext<'_>) -> Self::Return {
        println!("PRASER: {:?}", ctx.a());
        self.visit(&*ctx.a().unwrap())
    }

    fn visit_a(&mut self,ctx: &crate::parser::visitorbasicparser::AContext<'_>) -> Self::Return {
        println!("PRASER: {:?}", ctx.get_text());
        ctx.get_text().parse().expect("Failed to parse integer value")
        
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::visitorbasiclexer::VisitorBasicLexer;
    use crate::parser::visitorbasicparser::VisitorBasicParser;
    use crate::parser::visitorbasicparser::VisitorBasicParserContextType;
    use crate::parser::visitorbasicvisitor::VisitorBasicVisitorCompat;
    use antlr_rust::InputStream;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::tree::ErrorNode;

    #[test]
    fn test_visit_error_node() {
        let lexer = VisitorBasicLexer::new(InputStream::new(""));
        let mut parser = VisitorBasicParser::new(CommonTokenStream::new(lexer));

        let root = parser.s().unwrap();
        assert_eq!("(s <missing 'A'> <EOF>)", root.to_string_tree(&*parser));

        struct TestVisitor(String);
        impl ParseTreeVisitorCompat<'_> for TestVisitor {
            type Node = VisitorBasicParserContextType;
            type Return = String;

            fn temp_result(&mut self) -> &mut Self::Return {
                &mut self.0
            }

            fn visit_error_node(&mut self, _node: &ErrorNode<'_, Self::Node>) -> Self::Return {
                format!("Error encountered: {}", _node.symbol)
            }

            fn aggregate_results(
                &self,
                aggregate: Self::Return,
                next: Self::Return,
            ) -> Self::Return {
                aggregate + &next
            }
        }
        impl VisitorBasicVisitorCompat<'_> for TestVisitor {}

        let result = TestVisitor(String::new()).visit(&*root);
        let expected = "Error encountered: [@-1,-1:-1='<missing 'A'>',<1>,1:0]";
        assert_eq!(result, expected)
    }
}

```

## File: `src/lib.rs`

```rust
/// Includes the parser generated by the grammar
pub mod parser;

pub mod interpreter;

```

## File: `src/main.rs`

```rust
use antlr_rust::{
    InputStream,
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
};

use int_expr_interpreter::{
    interpreter::IntExprInterpreter,
    parser::{intexprlexer::IntExprLexer, intexprparser::IntExprParser},
};

fn main() {
    println!("Enter a string to parse:");

    // Get user input from stdin
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let input = InputStream::new(input_string.trim());

    // Create a TokenSource from the CharStream using the IntExpr grammar
    let lexer = IntExprLexer::new(input);

    // Obtain the tokens from the TokenSource as a TokenStream
    let tokens = CommonTokenStream::new(lexer);

    // Create a parser that parses the IntExpr grammar
    let mut parser = IntExprParser::new(tokens);

    // Execute the grammar from the 'main' nonterminal symbol
    let tree = parser.main().unwrap();

    let mut interpreter = IntExprInterpreter(0);
    let intperpreted_result = interpreter.visit(&*tree);

    println!("{}", tree.to_string_tree(&*parser));

    println!("Interpreted result = {intperpreted_result}");

    /* let lexer = int_expr_interpreter::parser::visitorbasiclexer::VisitorBasicLexer::new(InputStream::new(input_string.trim()));
    let mut parser = int_expr_interpreter::parser::visitorbasicparser::VisitorBasicParser::new(CommonTokenStream::new(lexer));

    let root = parser.s().unwrap();
    // assert_eq!("(s <missing 'A'> <EOF>)", root.to_string_tree(&*parser));

    let result = int_expr_interpreter::interpreter::TestVisitor(0).visit(&*root);
    // let expected = "Error encountered: [@-1,-1:-1='<missing 'A'>',<1>,1:0]";
    // assert_eq!(result, expected);

    println!("{result}"); */
}

```

## File: `src/parser.rs`

```rust
// Suppress all warnings from generated code
#![allow(warnings)]

pub mod intexprlexer {
    include!(concat!(env!("OUT_DIR"), "/intexprlexer.rs"));
}

pub mod intexprparser {
    include!(concat!(env!("OUT_DIR"), "/intexprparser.rs"));
}

pub mod intexprlistener {
    include!(concat!(env!("OUT_DIR"), "/intexprlistener.rs"));
}

pub mod intexprvisitor {
    include!(concat!(env!("OUT_DIR"), "/intexprvisitor.rs"));
}



pub mod visitorbasiclexer {
    include!(concat!(env!("OUT_DIR"), "/visitorbasiclexer.rs"));
}

pub mod visitorbasicparser {
    include!(concat!(env!("OUT_DIR"), "/visitorbasicparser.rs"));
}

pub mod visitorbasiclistener {
    include!(concat!(env!("OUT_DIR"), "/visitorbasiclistener.rs"));
}

pub mod visitorbasicvisitor {
    include!(concat!(env!("OUT_DIR"), "/visitorbasicvisitor.rs"));
}

```

## File: `Cargo.toml`

```toml
[package]
name = "int-expr-interpreter"
version = "0.1.0"
edition = "2024"

[dependencies]
antlr-rust = "0.3.0-beta"

```

## File: `README.md`

```markdown
# Rust antlr4 template

This is a template for using antlr4 with Rust. It includes just the necessary files to
generate all the code for a simple grammar to use in Rust.

## Usage

1. Create a new repo with this template

```
gh repo create --template fabiooo4/antlr4rust-template <repo-name>
```

2. Download required dependencies:
   - [antlr4 fork with Rust target support version 0.3.0-beta](https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta)
   - Java JDK

3. Add the path to the antlr4 jar file to the `ANTLR_JAR` environment variable

```
export ANTLR_JAR=/path/to/antlr4-4.8-2-SNAPSHOT-complete.jar
```

4. Build the project

```
cargo build
```

### Add new grammars

To add new grammars put them into the `grammars` folder and add the corresponding
file name to the `build.rs` file. The generated modules will be placed in the `target`
directory accessible by the `OUT_DIR` environment variable, so to use them
you need to include them in the `src/parser.rs` module. For example:

```rust
// src/parser.rs
#![allow(warnings)]

pub mod binwordslexer {
    include!(concat!(env!("OUT_DIR"), "/binwordslexer.rs"));
}

pub mod binwordsparser {
    include!(concat!(env!("OUT_DIR"), "/binwordsparser.rs"));
}

pub mod binwordslistener {
    include!(concat!(env!("OUT_DIR"), "/binwordslistener.rs"));
}
```

They will then be available as `parser::binwordslexer`, `parser::binwordsparser` and
`parser::binwordslistener` in the rest of the code.

```

## File: `build.rs`

```rust
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // List of grammar files with optional additional argument to be passed to the antlr tool
    let grammars: Vec<(&str, Option<Vec<&str>>)> = vec![
        ("IntExpr", Some(vec!["-visitor"])),
        ("VisitorBasic", Some(vec!["-visitor"])),
    ];

    let antlr_path = find_antlr_jar();

    for (grammar, arg) in grammars {
        if let Err(e) = gen_for_grammar(grammar, arg, &antlr_path) {
            panic!("Failed to generate parser for grammar '{}': {}", grammar, e);
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}

fn find_antlr_jar() -> PathBuf {
    // Check if the jar path env var is set
    println!("cargo:rerun-if-env-changed=ANTLR_JAR");
    if let Ok(path) = env::var("ANTLR_JAR") {
        println!("cargo:rerun-if-changed={path}");
        return PathBuf::from(path);
    }

    // fallback check common paths
    let fallback_paths = [
        "/usr/share/java/antlr4-4.8-2-SNAPSHOT-complete.jar",
        "/usr/local/lib/antlr4-4.8-2-SNAPSHOT-complete.jar",
        "/opt/homebrew/lib/antlr4-4.8-2-SNAPSHOT-complete.jar",
        "/usr/local/Cellar/antlr/4.8/antlr4-4.8-2-SNAPSHOT-complete.jar",
        "./antlr4-4.8-2-SNAPSHOT-complete.jar",
    ];

    for path in fallback_paths {
        if Path::new(path).exists() {
            println!("cargo:rerun-if-changed={path}");
            return PathBuf::from(path);
        }
    }

    panic!(
        "ANTLR tool fork with rust target not found! Please install it from, \n\
        https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta, \n\
        and set the ANTLR_JAR environment variable to point to the complete jar file. \n\
        Example: export ANTLR_JAR=/path/to/antlr4-4.8-2-SNAPSHOT-complete.jar"
    );
}

fn gen_for_grammar(
    grammar_file_name: &str,
    additional_arg: Option<Vec<&str>>,
    antlr_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let input_path = env::current_dir().unwrap().join("grammars");
    let file_name = input_path.join(grammar_file_name.to_owned() + ".g4");

    let out_dir = env::var("OUT_DIR");
    let dest_path = match out_dir.ok() {
        Some(path) => Path::new(&path).to_path_buf(),

        // Fallback
        None => env::current_dir().unwrap().join("src").join("gen"),
    };

    let output = Command::new("java")
        .current_dir(env::current_dir().unwrap())
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg(&dest_path)
        .arg(&file_name)
        .args(additional_arg.unwrap_or_default())
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        panic!(
            "ANTLR command failed with status {}.\n{}\n{}",
            output.status, stderr, stdout
        );
    }

    // Comment all inner allow attributes, they cannot be included by include! macro
    if let Ok(out_dir) = env::var("OUT_DIR") {
        for entry in std::fs::read_dir(out_dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let content = std::fs::read_to_string(&path).unwrap();

                // Only target the 'allow' attributes
                if content.contains("#![allow(") {
                    let fixed_content = content.replace("#![allow(", "// #![allow(");

                    std::fs::write(&path, fixed_content).unwrap();
                }
            }
        }
    }

    println!(
        "cargo:rerun-if-changed=grammars/{}",
        file_name.to_string_lossy()
    );
    Ok(())
}

```

