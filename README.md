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
