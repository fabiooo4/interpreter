# Basic interpreter

A simple interpreter for a basic programming language, implemented in Rust
using ANTLR4 for lexical analysis and parsing. This project is designed for educational
purposes.

## Language Features

- Variables: Support for declaration with `let` and mutation.
- Scoping: Dynamic scoping for variable resolution.
- Control Flow: `if`, `else`, and `while` loops.
- Data Types: Integers, Floats, Booleans, Strings, and Characters.
- Type System:
  - Explicit Casting: Convert between types using built-in functions.
  - Type Coercion: Automatic conversion during operations (e.g., mixing integers and floats).
- Operators:
  - Arithmetic: `+`, `-`, `*`, `/`, `mod`, `^` (power)
  - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
  - Logical: `and`, `or`, `not`
  - String: `:` (concatenation)
- Built-in Functions: `print()` for output and `to_str()` for type conversion.

## Usage

### Dependencies

- [antlr4 fork with Rust target support version 0.3.0-beta](https://github.com/rrevenantt/antlr4rust/releases/tag/antlr4-4.8-2-Rust0.3.0-beta)
  - Add the path to the antlr4 jar file to the `ANTLR_JAR` environment variable or put it in the project folder
- Java JDK

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
