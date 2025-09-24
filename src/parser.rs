use std::iter::Peekable;

use anyhow::anyhow;

use crate::characters::*;

#[derive(Debug, PartialEq)]
enum Step {
    AnyAlphanumerical,
    AnyDigit,
    EndOfString,
    ExactChar(char),
    MinOrMore(char, usize),
    CharGroup(Box<[char]>, bool),
}

struct Context {
    steps: Box<[Step]>,
}

impl Context {
    pub fn from(pattern: &str) -> Result<Self, anyhow::Error> {
        let mut steps = Vec::new();
        let mut iter = pattern.chars().peekable();
        while let Some(pattern_char) = iter.next() {
            match pattern_char {
                ANCHOR_END => steps.push(Step::EndOfString),
                ESCAPE_CHAR => {
                    if let Some(next) = iter.next() {
                        match Self::parse_escaped_char(next) {
                            Ok(step) => steps.push(step),
                            Err(e) => return Err(e),
                        }
                    } else {
                        return Err(anyhow!("Unhandled pattern"));
                    }
                }
                CHAR_GROUP_START => steps.push(Self::parse_char_group(iter.by_ref())),
                exact_char => steps.push(Self::parse_exact_char(exact_char, iter.by_ref())),
            };
        }
        Ok(Self {
            steps: steps.into_boxed_slice(),
        })
    }
}

impl Context {
    fn parse_escaped_char(escaped_char: char) -> Result<Step, anyhow::Error> {
        match escaped_char {
            CHAR_CLASS_ALPHANUMERIC => Ok(Step::AnyAlphanumerical),
            CHAR_CLASS_DIGIT => Ok(Step::AnyDigit),
            _ => {
                if ALLOWED_SPECIAL_CHARS.contains(&escaped_char) {
                    Ok(Step::ExactChar(escaped_char))
                } else {
                    Err(anyhow!("Unhandled escaped pattern \\{escaped_char}"))
                }
            }
        }
    }

    fn parse_exact_char<I>(exact_char: char, iter: &mut Peekable<I>) -> Step
    where
        I: Iterator<Item = char>,
    {
        if iter.next_if_eq(&QUANTIFIER_PLUS).is_some() {
            // Check how often the same pattern is found after +
            let mut n = 0;
            while iter.next_if_eq(&exact_char).is_some() {
                n += 1;
            }
            Step::MinOrMore(exact_char, 1 + n)
        } else {
            Step::ExactChar(exact_char)
        }
    }

    fn parse_char_group<I>(iter: &mut Peekable<I>) -> Step
    where
        I: Iterator<Item = char>,
    {
        let is_neg = iter.next_if(|c| *c == QUANTIFIER_NEG).is_some();
        let char_group = iter
            .take_while(|c| *c != CHAR_GROUP_END)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Step::CharGroup(char_group, is_neg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_one_or_more_times() {
        assert_eq!(
            Context::from("ca+t").unwrap().steps,
            vec![
                Step::ExactChar('c'),
                Step::MinOrMore('a', 1),
                Step::ExactChar('t')
            ]
            .into_boxed_slice()
        );
        assert_eq!(
            Context::from("ca+at").unwrap().steps,
            vec![
                Step::ExactChar('c'),
                Step::MinOrMore('a', 2),
                Step::ExactChar('t')
            ]
            .into_boxed_slice()
        );
    }

    /*#[test]
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
    }*/

    #[test]
    fn test_negative_character_groups() {
        assert_eq!(
            Context::from("[^xyz]").unwrap().steps,
            vec![Step::CharGroup(
                vec!['x', 'y', 'z'].into_boxed_slice(),
                true
            )]
            .into_boxed_slice()
        );
        assert_eq!(
            Context::from("[abc]").unwrap().steps,
            vec![Step::CharGroup(
                vec!['a', 'b', 'c'].into_boxed_slice(),
                false
            )]
            .into_boxed_slice()
        );
    }

    /*#[test]
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
    }*/
}
