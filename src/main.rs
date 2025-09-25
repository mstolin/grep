use std::env;
use std::io;
use std::process;

use crate::parser::Parser;
use crate::regex::Regex;

mod characters;
mod parser;
mod regex;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match Regex::from(pattern) {
        Ok(re) => Parser::parse(input_line, re),
        Err(e) => panic!("{e}"),
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
    fn test_wildcard() {
        assert!(match_pattern("cat", "c.t"));
        assert!(!match_pattern("car", "c.t"));
        assert!(match_pattern("goøö0Ogol", "g.+gol"));
        assert!(!match_pattern("gol", "g.+gol"));
    }

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
