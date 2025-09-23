use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let chars = &pattern.chars().collect::<Vec<_>>()[..];

    // TODO This is ugly
    match pattern {
        "\\d" => input_line.contains(char::is_numeric),
        "\\w" => input_line
            .chars()
            .any(|c| c.is_ascii_alphanumeric() || c == '_'),
        _ => match chars {
            ['[', '^', ncg @ .., ']'] => input_line.chars().any(|c| !ncg.contains(&c)),
            ['[', pcg @ .., ']'] => input_line.contains(|c| pcg.contains(&c)),
            _ => {
                if pattern.chars().count() == 1 {
                    input_line.contains(pattern)
                } else {
                    panic!("Unhandled pattern: {}", pattern)
                }
            }
        },
    }
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
