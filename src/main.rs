use std::env;
use std::io;
use std::process;

use anyhow::anyhow;

fn do_match(
    mut input_iter: impl Iterator<Item = char>,
    mut pattern_iter: impl Iterator<Item = char>,
) -> Result<bool, anyhow::Error> {
    while let Some(pattern_char) = pattern_iter.next() {
        if pattern_char == '\\' {
            let input_char = if let Some(input_char) = input_iter.next() {
                input_char
            } else {
                return Err(anyhow!("Input has unsufficient length"));
            };
            match pattern_iter.next() {
                Some('d') => {
                    if !input_char.is_ascii_digit() {
                        return Ok(false);
                    }
                }
                Some('w') => {
                    if !(input_char.is_ascii_alphanumeric() || input_char == '_') {
                        return Ok(false);
                    }
                }
                Some('\\') => {
                    // input is a special char that requires escape
                    // TODO check allows special chars
                    if input_char != '\\' {
                        return Ok(false);
                    }
                }
                _ => return Err(anyhow!("Unhandled pattern")),
            }
        } else if pattern_char == '[' {
            // check for character group
            let mut char_group = pattern_iter
                .by_ref()
                .take_while(|c| *c != ']')
                .collect::<Vec<_>>();
            let is_neg = if let Some(pattern_first) = char_group.first() {
                *pattern_first == '^'
            } else {
                false
            };
            let res = if is_neg {
                char_group.remove(0);
                input_iter.any(|c| !char_group.contains(&c))
            } else {
                input_iter.any(|c| char_group.contains(&c))
            };
            return Ok(res);
        } else if pattern_char.is_ascii() {
            let input_char = if let Some(input_char) = input_iter.next() {
                input_char
            } else {
                return Err(anyhow!("Input has unsufficient length"));
            };
            // check for the exact match
            if pattern_char != input_char {
                return Ok(false);
            }
        } else {
            return Err(anyhow!("Unhandled pattern"));
        }
    }
    // Iterators have to be consumed
    Ok(input_iter.count() == 0 && pattern_iter.count() == 0)
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let input_iter = input_line.chars();
    let pattern_iter = pattern.chars();

    match do_match(input_iter, pattern_iter) {
        Ok(res) => {
            println!("Result: {res}");
            res
        }
        Err(e) => panic!("{e}: {pattern}"),
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
