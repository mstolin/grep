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
mod walker;

fn match_line(input_line: &str, pattern: &str) -> Result<bool, anyhow::Error> {
    let re = Regex::from(pattern)?;
    Ok(Parser::parse(input_line, re))
}

fn match_file_lines(
    mut lines: Lines<BufReader<File>>,
    pattern: &str,
) -> Result<Box<[String]>, anyhow::Error> {
    let mut matched_lines = Vec::new();
    while let Some(input_line) = lines.next() {
        let input_line = input_line?;
        if match_line(&input_line, &pattern)? {
            matched_lines.push(input_line);
        }
    }
    Ok(matched_lines.into_boxed_slice())
}

fn read_file(path: PathBuf) -> Result<BufReader<File>, anyhow::Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

// Usage: echo -n <input_text> | your_program -E <pattern>
//        your_program -E <pattern> <input_file>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();

    let matched_lines = if env::args().len() >= 3 {
        // TODO Don't use bufreader here
        let mut args_iter = env::args().into_iter().skip(3);
        let mut all_matches = Vec::new();
        while let Some(file_path) = args_iter.next() {
            let reader = read_file(file_path.into()).unwrap_or_else(|err| panic!("{err}"));
            match match_file_lines(reader.lines(), &pattern) {
                Ok(matches) => {
                    let mut matches_iter = matches.iter();
                    while let Some(next) = matches_iter.next() {
                        all_matches.push(next.into());
                    }
                }
                Err(err) => panic!("{err}"),
            }
        }
        all_matches.into_boxed_slice()
    } else {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        match match_line(&input_line, &pattern) {
            Ok(res) => {
                if res {
                    vec![input_line].into_boxed_slice()
                } else {
                    Box::new([])
                }
            }
            Err(err) => panic!("{err}"),
        }
    };

    if !matched_lines.is_empty() {
        for line in matched_lines {
            println!("{line}")
        }
        process::exit(0)
    } else {
        process::exit(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternation() {
        assert!(match_line("a cat", "a (cat|dog)").unwrap());
        assert!(!match_line("a cow", "a (cat|dog)").unwrap());
        assert!(match_line("I see 1 cat", r"^I see \d+ (cat|dog)s?$").unwrap());
        assert!(!match_line("gol", "g.+gol").unwrap());
    }

    #[test]
    fn test_wildcard() {
        assert!(match_line("cat", "c.t").unwrap());
        assert!(!match_line("car", "c.t").unwrap());
        assert!(match_line("goøö0Ogol", "g.+gol").unwrap());
        assert!(!match_line("gol", "g.+gol").unwrap());
    }

    #[test]
    fn test_match_one_or_more_times() {
        assert!(match_line("cat", "ca+t").unwrap());
        assert!(match_line("caaats", "ca+at").unwrap());
        assert!(!match_line("act", "ca+t").unwrap());
        assert!(!match_line("ca", "ca+t").unwrap());
    }

    #[test]
    fn test_end_of_string_anchor() {
        assert!(match_line("grape_raspberry", "raspberry$").unwrap());
        assert!(!match_line("raspberry_grape", "raspberry$").unwrap());
        assert!(match_line("mango", "^mango$").unwrap());
        assert!(!match_line("mango_mango", "^mango$").unwrap());
    }

    #[test]
    fn test_start_of_string_anchor() {
        assert!(match_line("apple_mango", "^apple").unwrap());
        assert!(!match_line("mango_apple", "^apple").unwrap());
    }

    #[test]
    fn test_combining_character_classes() {
        assert!(match_line("sally has 3 apples", r"\d apple").unwrap());
        assert!(!match_line("sally has 1 orange", r"\d apple").unwrap());
        assert!(match_line("sally has 124 apples", r"\d\d\d apples").unwrap());
        assert!(!match_line("sally has 12 apples", r"\d\d\d apples").unwrap());
        assert!(match_line("sally has 3 dogs", r"\d \w\w\ws").unwrap());
        assert!(match_line("sally has 4 dogs", r"\d \w\w\ws").unwrap());
        assert!(!match_line("sally has 1 dog", r"\d \w\w\ws").unwrap());
    }

    #[test]
    fn test_negative_character_groups() {
        assert!(match_line("apple", "[^xyz]").unwrap());
        assert!(match_line("apple", "[^abc]").unwrap());
        assert!(!match_line("banana", "[^anb]").unwrap());
        assert!(match_line("orange", "[^opq]").unwrap());
    }

    #[test]
    fn test_positive_character_groups() {
        assert!(match_line("u", "[blueberry]").unwrap());
        assert!(match_line("uac", "[blueberry]").unwrap());
        assert!(!match_line("orange", "[bcdfhi]").unwrap());
        assert!(!match_line("[]", "[pear]").unwrap());
    }

    #[test]
    fn test_match_word_characters() {
        assert!(match_line("blueberry", r"\w").unwrap());
        assert!(match_line("BANANA", r"\w").unwrap());
        assert!(match_line("656", r"\w").unwrap());
        assert!(match_line("#+%_+#×", r"\w").unwrap());
        assert!(!match_line("=#÷+×-", r"\w").unwrap());
    }

    #[test]
    fn test_match_digits() {
        assert!(match_line("123", r"\d").unwrap());
        assert!(!match_line("apple", r"\d").unwrap());
    }

    #[test]
    fn test_match_literal_character() {
        assert!(match_line("dog", "d").unwrap());
        assert!(!match_line("dog", "f").unwrap());
    }
}
