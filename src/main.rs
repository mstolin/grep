use std::char;
use std::env;
use std::io;
use std::iter::Peekable;
use std::process;

use crate::characters::*;
use anyhow::anyhow;

mod characters;
mod parser;

/* TODO List:
  [] Custom structs for specifc pattern types (e.g. character groups)
  [] Custom error types
  [] Make iterators AsRef
*/

enum ModeLiteralMatch {
    One,
    OneOrMore,
    ZeroOrOne,
}

impl ModeLiteralMatch {
    fn from(c: char) -> Self {
        match c {
            QUANTIFIER_PLUS => Self::OneOrMore,
            QUANTIFIER_QUESTION => Self::ZeroOrOne,
            _ => Self::One,
        }
    }
}

fn match_escaped(
    escaped_char: char,
    input_iter: impl Iterator<Item = char>,
) -> Result<bool, anyhow::Error> {
    match escaped_char {
        CHAR_CLASS_ALPHANUMERIC => Ok(until_alphanumeric(input_iter)),
        CHAR_CLASS_DIGIT => Ok(until_digit(input_iter)),
        symbol => {
            if ALLOWED_SPECIAL_CHARS.contains(&symbol) {
                Ok(until_exact(&symbol, input_iter))
            } else {
                Err(anyhow!("Unhandled pattern \\{symbol}"))
            }
        }
    }
}

fn match_one_or_more_literals<I: Iterator<Item = char>>(
    literal: &char,
    input_peek: &mut Peekable<I>,
) -> bool {
    let mut matches = 0;
    while let Some(peek) = input_peek.peek() {
        if *peek == *literal {
            matches += 1;
            let _ = input_peek.next();
        } else {
            break;
        }
    }
    matches > 0
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

fn until_digit(input_iter: impl Iterator<Item = char>) -> bool {
    until_match(input_iter, |c| c.is_ascii_digit())
}

fn until_alphanumeric(input_iter: impl Iterator<Item = char>) -> bool {
    until_match(input_iter, |c| c.is_ascii_alphanumeric() || c == '_')
}

fn until_exact(pattern: &char, input_iter: impl Iterator<Item = char>) -> bool {
    until_match(input_iter, |c| c == *pattern)
}

fn do_match(
    input_iter: impl Iterator<Item = char>,
    pattern_iter: impl Iterator<Item = char>,
) -> Result<bool, anyhow::Error> {
    let mut pattern_peek = pattern_iter.peekable();
    let mut input_peek = input_iter.peekable();

    while let Some(pattern_char) = pattern_peek.next() {
        if pattern_char == ANCHOR_END {
            // next char must be end of string
            return Ok(input_peek.next().is_none());
        } else if pattern_char == ESCAPE_CHAR {
            // Match escaped char
            if let Some(next_pattern) = pattern_peek.next() {
                if !match_escaped(next_pattern, input_peek.by_ref())? {
                    return Ok(false);
                }
            } else {
                return Err(anyhow!(r"no symbol after \"));
            }
        } else if pattern_char == CHAR_GROUP_START {
            let is_neg =
                if let Some(QUANTIFIER_NEG) = pattern_peek.next_if(|c| *c == QUANTIFIER_NEG) {
                    true
                } else {
                    false
                };
            // check for character group
            let char_group = pattern_peek
                .by_ref()
                .take_while(|c| *c != CHAR_GROUP_END)
                .collect::<Vec<_>>();

            let res = if is_neg {
                input_peek.any(|c| !char_group.contains(&c))
            } else {
                input_peek.any(|c| char_group.contains(&c))
            };
            return Ok(res);
        } else if pattern_char.is_ascii() {
            // Match literal char
            let mode = if let Some(symbol) = pattern_peek.peek() {
                ModeLiteralMatch::from(*symbol)
            } else {
                ModeLiteralMatch::One
            };

            match mode {
                ModeLiteralMatch::One => {
                    if !until_exact(&pattern_char, input_peek.by_ref()) {
                        return Ok(false);
                    }
                }
                ModeLiteralMatch::OneOrMore => {
                    let matches = match_one_or_more_literals(&pattern_char, input_peek.by_ref());
                    if matches {
                        // Increase pattern iterator until a different symbol is found
                        let _ = pattern_peek.next();
                        while let Some(peek) = pattern_peek.peek() {
                            if *peek == pattern_char {
                                let _ = pattern_peek.next();
                            } else {
                                break;
                            }
                        }
                    } else {
                        return Ok(false);
                    }
                }
                ModeLiteralMatch::ZeroOrOne => todo!(),
            }
        } else {
            return Err(anyhow!("Unhandled pattern"));
        }
    }
    Ok(true)
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
        println!("Y");
        process::exit(0)
    } else {
        println!("N");
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_one_or_more_times() {
        assert!(match_pattern("cat", "ca+t"));
        assert!(match_pattern("caaats", "ca+at"));
        assert!(!match_pattern("act", "ca+t"));
        assert!(!match_pattern("ca", "ca+t"));
    }

    #[test]
    fn test_end_of_string_anchor() {
        assert!(match_pattern("grape_raspberry", "raspberry$"));
        assert!(!match_pattern("raspberry_grape", "raspberry$"));
        assert!(match_pattern("mango", "^mango$"));
        assert!(!match_pattern("mango_mango", "^mango$"));
    }

    #[test]
    fn test_start_of_string_anchor() {
        assert!(match_pattern("apple_mango", "^apple"));
        assert!(!match_pattern("mango_apple", "^apple"));
    }

    #[test]
    fn test_combining_character_classes() {
        assert!(match_pattern("sally has 3 apples", r"\d apple"));
        assert!(!match_pattern("sally has 1 orange", r"\d apple"));
        assert!(match_pattern("sally has 124 apples", r"\d\d\d apples"));
        assert!(!match_pattern("sally has 12 apples", r"\d\d\d apples"));
        assert!(match_pattern("sally has 3 dogs", r"\d \w\w\ws"));
        assert!(match_pattern("sally has 4 dogs", r"\d \w\w\ws"));
        assert!(!match_pattern("sally has 1 dog", r"\d \w\w\ws"));
    }

    #[test]
    fn test_negative_character_groups() {
        assert!(match_pattern("apple", "[^xyz]"));
        assert!(match_pattern("apple", "[^abc]"));
        assert!(!match_pattern("banana", "[^anb]"));
        assert!(match_pattern("orange", "[^opq]"));
    }

    #[test]
    fn test_positive_character_groups() {
        assert!(match_pattern("u", "[blueberry]"));
        assert!(match_pattern("uac", "[blueberry]"));
        assert!(!match_pattern("orange", "[bcdfhi]"));
        assert!(!match_pattern("[]", "[pear]"));
    }

    #[test]
    fn test_match_word_characters() {
        assert!(match_pattern("blueberry", r"\w"));
        assert!(match_pattern("BANANA", r"\w"));
        assert!(match_pattern("656", r"\w"));
        assert!(match_pattern("#+%_+#×", r"\w"));
        assert!(!match_pattern("=#÷+×-", r"\w"));
    }

    #[test]
    fn test_match_digits() {
        assert!(match_pattern("123", r"\d"));
        assert!(!match_pattern("apple", r"\d"));
    }

    #[test]
    fn test_match_literal_character() {
        assert!(match_pattern("dog", "d"));
        assert!(!match_pattern("dog", "f"));
    }
}
