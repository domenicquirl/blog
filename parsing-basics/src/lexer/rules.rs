use crate::T;

use super::TokenKind;

pub struct Rule {
    pub kind:    TokenKind,
    pub matches: fn(&str) -> Option<usize>,
}

/// If the given character is a character that _only_ represents a token of length 1,
/// this method returns the corresponding `TokenKind`.
/// Note that this method will return `None` for characters like `=` that may also
/// occur at the first position of longer tokens (here `==`).
pub(crate) const fn unambiguous_single_char(c: char) -> Option<TokenKind> {
    Some(match c {
        '+' => T![+],
        '-' => T![-],
        '*' => T![*],
        '/' => T![/],
        '^' => T![^],
        '.' => T![.],
        ',' => T![,],
        '<' => T![<],
        '>' => T![>],
        '[' => T!['['],
        ']' => T![']'],
        '{' => T!['{'],
        '}' => T!['}'],
        '(' => T!['('],
        ')' => T![')'],
        ':' => T![:],
        _ => return None,
    })
}

// pub(crate) fn get_rules() -> Vec<Rule> {
//     todo!();
//     vec![
//         Rule {
//             kind:    TokenKind::Mod,
//             matches: |input| if input.starts_with("mod") { Some(3) } else { None },
//         },
//         Rule {
//             kind:    TokenKind::Underscore,
//             matches: |input| match_single_char(input, '_'),
//         },
//         Rule {
//             kind:    TokenKind::Pound,
//             matches: |input| match_single_char(input, '#'),
//         },
//         Rule {
//             kind:    TokenKind::Dollar,
//             matches: |input| match_single_char(input, '$'),
//         },
//         Rule {
//             kind:    TokenKind::Quote,
//             matches: |input| match_single_char(input, '"'),
//         },
//         Rule {
//             kind:    TokenKind::String,
//             matches: |input| match_regex(input, &STRING_REGEX),
//         },
//         Rule {
//             kind:    TokenKind::Comment,
//             matches: |input| match_regex(input, &COMMENT_REGEX),
//         },
//         Rule {
//             kind:    TokenKind::Int,
//             matches: |input| {
//                 input
//                     .char_indices()
//                     .take_while(|(_, c)| c.is_ascii_digit())
//                     .last()
//                     .map(|(pos, _)| pos + 1)
//             },
//         },
//         Rule {
//             kind:    TokenKind::Float,
//             matches: |input| match_regex(input, &FLOAT_REGEX),
//         },
//         Rule {
//             kind:    TokenKind::Identifier,
//             matches: |input| match_regex(input, &IDENTIFIER_REGEX),
//         },
//     ]
// }
