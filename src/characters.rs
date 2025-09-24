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
pub const ALLOWED_SPECIAL_CHARS: [char; 7] = [
    CHAR_GROUP_START,
    CHAR_GROUP_END,
    ESCAPE_CHAR,
    ANCHOR_START,
    ANCHOR_END,
    QUANTIFIER_PLUS,
    QUANTIFIER_QUESTION,
];
