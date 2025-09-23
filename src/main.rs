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
    } else if let Some(ncg) = extract_ncg(pattern) {
        // Does input contain any char, that is not in ncg? -> true
        input_line.chars().any(|c| !ncg.contains(c))
    } else if let Some(pcg) = extract_pcg(pattern) {
        input_line.contains(|c| pcg.contains(c))
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

/// Checks if `pattern` is a positive character group.
/// Example: [abc]
fn is_pcg(pattern: &str) -> bool {
    pattern.starts_with('[') && pattern.ends_with(']') && pattern.len() > 2 && !is_ncg(pattern)
}

/// Checks if `pattern` is a negative character group.
/// Example: [^abc]
fn is_ncg(pattern: &str) -> bool {
    let mut chars = pattern.chars();
    chars.next() == Some('[')
        && chars.next() == Some('^')
        && chars.last() == Some(']')
        && pattern.len() > 3
}

/// Extracts the characters from the positive character group.
fn extract_pcg(pattern: &str) -> Option<&str> {
    if !is_pcg(pattern) {
        return None;
    }
    let end = pattern.len() - 1;
    Some(&pattern[1..end])
}

/// Extracts the characters from the negative character group.
fn extract_ncg(pattern: &str) -> Option<&str> {
    if !is_ncg(pattern) {
        return None;
    }
    let end = pattern.len() - 1;
    Some(&pattern[2..end])
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
