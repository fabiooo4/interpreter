# Basic interpreter

A simple interpreter for a basic programming language, implemented in Rust
using ANTLR4 for lexical analysis and parsing. This project is designed for educational
purposes.

## Language Features

- Variables: Support for declaration with `let` and mutation.
- Control Flow: `if`, `else`, and `while` loops.
- Data Types: Integers, Floats, Booleans, Strings, and Characters.
- Operators:
    - Arithmetic: `+`, `-`, `*`, `/`, `mod`, `^` (power)
    - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
    - Logical: `and`, `or`, `not`
    - String: `:` (concatenation)
- Built-in Functions: `print()` for output and `to_str()` for type conversion.

## Usage

Ensure you have Rust and Cargo installed on your system.

### Running from a File

To execute a program stored in a file:

```bash
cargo run -- path/to/program
```

### Running from Standard Input

If no file path is provided, the interpreter will prompt for a single line of input:

```bash
cargo run
```

## Example Program

```rust
let n = 10;
let i = 0;
let sum = 0;

while i < n {
    sum = sum + i;
    i = i + 1;
}

print("The sum is: ");
print(sum);
```
