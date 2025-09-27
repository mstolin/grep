use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::{BufReader, Lines};
use std::path::PathBuf;
use std::process;

use crate::parser::Parser;
use crate::regex::Regex;

mod characters;
mod parser;
mod regex;

fn match_line(input_line: &str, pattern: &str) -> bool {
    match Regex::from(pattern) {
        Ok(re) => {
            let res = Parser::parse(input_line, re);
            if res {
                println!("{input_line}");
            }
            res
        }
        Err(e) => panic!("{e}"),
    }
}

fn match_file_lines(mut lines: Lines<BufReader<File>>, pattern: &str) -> std::io::Result<bool> {
    while let Some(input_line) = lines.next() {
        if !match_line(&input_line?, &pattern) {
            return Ok(false);
        }
    }
    Ok(true)
}

fn read_file(path: PathBuf) -> Result<BufReader<File>, anyhow::Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();

    let res = if let Some(input_path) = env::args().nth(3) {
        let reader = read_file(input_path.into()).unwrap_or_else(|err| panic!("{err}"));
        match match_file_lines(reader.lines(), &pattern) {
            Ok(res) => res,
            Err(err) => panic!("{err}"),
        }
    } else {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        match_line(&input_line, &pattern)
    };

    if res {
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
    fn test_alternation() {
        assert!(match_line("a cat", "a (cat|dog)"));
        assert!(!match_line("a cow", "a (cat|dog)"));
        assert!(match_line("I see 1 cat", r"^I see \d+ (cat|dog)s?$"));
        assert!(!match_line("gol", "g.+gol"));
    }

    #[test]
    fn test_wildcard() {
        assert!(match_line("cat", "c.t"));
        assert!(!match_line("car", "c.t"));
        assert!(match_line("goøö0Ogol", "g.+gol"));
        assert!(!match_line("gol", "g.+gol"));
    }

    #[test]
    fn test_match_one_or_more_times() {
        assert!(match_line("cat", "ca+t"));
        assert!(match_line("caaats", "ca+at"));
        assert!(!match_line("act", "ca+t"));
        assert!(!match_line("ca", "ca+t"));
    }

    #[test]
    fn test_end_of_string_anchor() {
        assert!(match_line("grape_raspberry", "raspberry$"));
        assert!(!match_line("raspberry_grape", "raspberry$"));
        assert!(match_line("mango", "^mango$"));
        assert!(!match_line("mango_mango", "^mango$"));
    }

    #[test]
    fn test_start_of_string_anchor() {
        assert!(match_line("apple_mango", "^apple"));
        assert!(!match_line("mango_apple", "^apple"));
    }

    #[test]
    fn test_combining_character_classes() {
        assert!(match_line("sally has 3 apples", r"\d apple"));
        assert!(!match_line("sally has 1 orange", r"\d apple"));
        assert!(match_line("sally has 124 apples", r"\d\d\d apples"));
        assert!(!match_line("sally has 12 apples", r"\d\d\d apples"));
        assert!(match_line("sally has 3 dogs", r"\d \w\w\ws"));
        assert!(match_line("sally has 4 dogs", r"\d \w\w\ws"));
        assert!(!match_line("sally has 1 dog", r"\d \w\w\ws"));
    }

    #[test]
    fn test_negative_character_groups() {
        assert!(match_line("apple", "[^xyz]"));
        assert!(match_line("apple", "[^abc]"));
        assert!(!match_line("banana", "[^anb]"));
        assert!(match_line("orange", "[^opq]"));
    }

    #[test]
    fn test_positive_character_groups() {
        assert!(match_line("u", "[blueberry]"));
        assert!(match_line("uac", "[blueberry]"));
        assert!(!match_line("orange", "[bcdfhi]"));
        assert!(!match_line("[]", "[pear]"));
    }

    #[test]
    fn test_match_word_characters() {
        assert!(match_line("blueberry", r"\w"));
        assert!(match_line("BANANA", r"\w"));
        assert!(match_line("656", r"\w"));
        assert!(match_line("#+%_+#×", r"\w"));
        assert!(!match_line("=#÷+×-", r"\w"));
    }

    #[test]
    fn test_match_digits() {
        assert!(match_line("123", r"\d"));
        assert!(!match_line("apple", r"\d"));
    }

    #[test]
    fn test_match_literal_character() {
        assert!(match_line("dog", "d"));
        assert!(!match_line("dog", "f"));
    }
}
