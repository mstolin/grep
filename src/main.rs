use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern.contains(r"\d") {
        input_line.contains(char::is_numeric)
    } else if pattern.contains(r"\w") {
        input_line
            .chars()
            .any(|c| c.is_ascii_alphanumeric() || c == '_')
    } else if let Some(pcg) = extract_pcg(pattern) {
        input_line.contains(|c| pcg.contains(c))
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

/// Extracts the positive character group from the pattern.
fn extract_pcg(pattern: &str) -> Option<&str> {
    if pattern.starts_with('[') && pattern.ends_with(']') && pattern.len() > 2 {
        let chars = pattern
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap();
        return Some(chars);
    }
    None
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
