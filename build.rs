use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // List of grammar files with optional additional argument to be passed to the antlr tool
    let grammars: Vec<(&str, Option<&str>)> = vec![("IntExpr", None)];

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
    additional_arg: Option<&str>,
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
        .args(additional_arg)
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
