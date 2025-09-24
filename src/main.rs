use std::char;
use std::env;
use std::io;
use std::process;

use anyhow::anyhow;

/* TODO List:
  [] Custom structs for specifc pattern types (e.g. character groups)
  [] Custom error types
  [] Make iterators AsRef
*/

const ALLOWED_SPECIAL_CHARS: [char; 6] = ['[', ']', '\\', '^', '$', '+'];

fn do_match(
    input_iter: impl Iterator<Item = char>,
    pattern_iter: impl Iterator<Item = char>,
) -> Result<bool, anyhow::Error> {
    let mut pattern_peek = pattern_iter.peekable();
    let mut input_peek = input_iter.peekable();

    while let Some(pattern_char) = pattern_peek.next() {
        if pattern_char == '$' {
            // end of string
            return Ok(input_peek.next().is_none());
        } else if pattern_char == '\\' {
            match pattern_peek.next() {
                Some('d') => {
                    if !until_digit(input_peek.by_ref()) {
                        return Ok(false);
                    }
                }
                Some('w') => {
                    if !until_alphanumeric(input_peek.by_ref()) {
                        return Ok(false);
                    }
                }
                Some(symbol) => {
                    // input is a special char that requires escape
                    if !(ALLOWED_SPECIAL_CHARS.contains(&symbol)
                        && until_exact(&symbol, input_peek.by_ref()))
                    {
                        return Ok(false);
                    }
                }
                _ => return Err(anyhow!("Unhandled pattern")),
            }
        } else if pattern_char == '[' {
            // check for character group
            let mut char_group = pattern_peek
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
                input_peek.any(|c| !char_group.contains(&c))
            } else {
                input_peek.any(|c| char_group.contains(&c))
            };
            return Ok(res);
        } else if pattern_char.is_ascii() {
            if let Some('+') = pattern_peek.peek() {
                // Increase input iterator as long as symbol is equal to pattern
                let mut matches = 0;
                while let Some(peek) = input_peek.peek() {
                    if *peek == pattern_char {
                        matches += 1;
                        let _ = input_peek.next();
                    } else {
                        break;
                    }
                }

                if matches >= 1 {
                    // Increase pattern iterator until a different symbol is found
                    let _ = pattern_peek.next();
                    while let Some(peek) = pattern_peek.peek() {
                        if *peek == pattern_char {
                            let _ = pattern_peek.next();
                        } else {
                            break;
                        }
                    }
                }
            } else {
                if !until_exact(&pattern_char, input_peek.by_ref()) {
                    return Ok(false);
                }
            }
        } else {
            return Err(anyhow!("Unhandled pattern"));
        }
    }
    Ok(true)
}

fn until_match<F>(mut input_iter: impl Iterator<Item = char>, is_match: F) -> bool
where
    F: Fn(char) -> bool,
{
    while let Some(input_char) = input_iter.next() {
        if is_match(input_char) {
            return true;
        }
    }
    false
}

// TODO Make as ref
fn until_digit(input_iter: impl Iterator<Item = char>) -> bool {
    until_match(input_iter, |c| c.is_ascii_digit())
}

// TODO Make as ref
fn until_alphanumeric(input_iter: impl Iterator<Item = char>) -> bool {
    until_match(input_iter, |c| c.is_ascii_alphanumeric() || c == '_')
}

// TODO Make as ref
fn until_exact(pattern: &char, input_iter: impl Iterator<Item = char>) -> bool {
    until_match(input_iter, |c| c == *pattern)
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut input_iter = input_line.chars();
    let mut pattern_iter = pattern.chars();

    if pattern.starts_with('^') {
        let _ = pattern_iter.next();
        while let (Some(input_char), Some(pattern_char)) = (input_iter.next(), pattern_iter.next())
        {
            if input_char != pattern_char {
                return false;
            }
        }
    }

    match do_match(input_iter, pattern_iter) {
        Ok(res) => res,
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
