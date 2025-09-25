use crate::regex::{Regex, Step, WildcardMatchMode};

pub struct Parser;

impl Parser {
    pub fn parse(input: &str, regex: Regex) -> bool {
        let mut input_chars = input.chars().peekable();
        let mut regex_iter = regex.into_iter().enumerate();
        while let Some((index, step)) = regex_iter.next() {
            match step {
                Step::Alternation(a, b) => {
                    let mut a_matches = 0;
                    let mut b_matches = 0;
                    while let Some(next) = input_chars.next() {
                        if let Some(a_next) = a.get(a_matches) {
                            if next == *a_next {
                                a_matches += 1;
                                if a_matches == a.len() {
                                    break;
                                }
                            } else {
                                a_matches = 0;
                            }
                        }
                        if let Some(b_next) = b.get(b_matches) {
                            if next == *b_next {
                                b_matches += 1;
                                if b_matches == b.len() {
                                    break;
                                }
                            } else {
                                b_matches = 0;
                            }
                        }
                    }
                    if !(a_matches == a.len() || b_matches == b.len()) {
                        return false;
                    }
                }
                Step::AnyAlphanumerical => {
                    if !Self::until_alphanumeric(input_chars.by_ref()) {
                        return false;
                    }
                }
                Step::AnyDigit => {
                    if !Self::until_digit(input_chars.by_ref()) {
                        return false;
                    }
                }
                Step::CharGroup(group, is_neg) => {
                    let matches = if is_neg {
                        input_chars.any(|c| !group.contains(&c))
                    } else {
                        input_chars.any(|c| group.contains(&c))
                    };
                    if !matches {
                        return false;
                    }
                }
                Step::EndOfString => {
                    let next = input_chars.next();
                    return next.is_none();
                }
                Step::ExactChar(c) => {
                    if !Self::until_exact(&c, input_chars.by_ref()) {
                        return false;
                    }
                }
                Step::MinOrMore(c, min) => {
                    let mut n = 0;
                    while let Some(_) = input_chars.next_if(|peek_char| *peek_char == c) {
                        n += 1;
                    }
                    if n < min {
                        return false;
                    }
                }
                Step::StartOfString(group) => {
                    if index > 0 {
                        return false;
                    }
                    let mut group_iter = group.iter();
                    while let Some(next_pattern) = group_iter.next() {
                        if let Some(input_char) = input_chars.next() {
                            if input_char != *next_pattern {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    }
                }
                Step::Wildcard(mode) => {
                    match mode {
                        WildcardMatchMode::Single => {
                            if input_chars.next().is_none() {
                                return false;
                            }
                        }
                        WildcardMatchMode::Until(c) => {
                            while input_chars.next_if(|n| *n != c).is_some() {}
                        }
                        WildcardMatchMode::Endless => while input_chars.next().is_some() {},
                    };
                }
                Step::ZeroOrOne(c) => {
                    input_chars.next_if(|peek_char| *peek_char == c);
                }
            }
        }
        true
    }
}

impl Parser {
    fn until_match<F>(mut iter: impl Iterator<Item = char>, is_match: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        while let Some(input_char) = iter.next() {
            if is_match(input_char) {
                return true;
            }
        }
        false
    }

    fn until_digit(iter: impl Iterator<Item = char>) -> bool {
        Self::until_match(iter, |c| c.is_ascii_digit())
    }

    fn until_alphanumeric(iter: impl Iterator<Item = char>) -> bool {
        Self::until_match(iter, |c| c.is_ascii_alphanumeric() || c == '_')
    }

    fn until_exact(pattern: &char, iter: impl Iterator<Item = char>) -> bool {
        Self::until_match(iter, |c| c == *pattern)
    }
}
