pub const ALTERNATION: char = '|';
pub const ALTERNATION_START: char = '(';
pub const ALTERNATION_END: char = ')';
pub const ANCHOR_START: char = '^';
pub const ANCHOR_END: char = '$';
pub const CHAR_CLASS_ALPHANUMERIC: char = 'w';
pub const CHAR_CLASS_DIGIT: char = 'd';
pub const CHAR_GROUP_START: char = '[';
pub const CHAR_GROUP_END: char = ']';
pub const ESCAPE_CHAR: char = '\\';
pub const QUANTIFIER_NEG: char = '^';
pub const QUANTIFIER_PLUS: char = '+';
pub const QUANTIFIER_QUESTION: char = '?';
pub const WILDCARD: char = '.';
pub const SPECIAL_CHARS: [char; 11] = [
    ALTERNATION,
    ALTERNATION_START,
    ALTERNATION_END,
    CHAR_GROUP_START,
    CHAR_GROUP_END,
    ESCAPE_CHAR,
    ANCHOR_START,
    ANCHOR_END,
    QUANTIFIER_PLUS,
    QUANTIFIER_QUESTION,
    WILDCARD,
];
