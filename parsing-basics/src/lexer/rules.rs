use super::TokenKind;

pub struct Rule {
    pub kind:    TokenKind,
    pub matches: fn(&str) -> Option<usize>,
}

pub(crate) fn get_rules() -> Vec<Rule> {
    todo!();
    vec![
        Rule {
            kind:    TokenKind::Mod,
            matches: |input| if input.starts_with("mod") { Some(3) } else { None },
        },
        Rule {
            kind:    TokenKind::Underscore,
            matches: |input| match_single_char(input, '_'),
        },
        Rule {
            kind:    TokenKind::Pound,
            matches: |input| match_single_char(input, '#'),
        },
        Rule {
            kind:    TokenKind::Dollar,
            matches: |input| match_single_char(input, '$'),
        },
        Rule {
            kind:    TokenKind::Quote,
            matches: |input| match_single_char(input, '"'),
        },
        Rule {
            kind:    TokenKind::String,
            matches: |input| match_regex(input, &STRING_REGEX),
        },
        Rule {
            kind:    TokenKind::Comment,
            matches: |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind:    TokenKind::Int,
            matches: |input| {
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_ascii_digit())
                    .last()
                    .map(|(pos, _)| pos + 1)
            },
        },
        Rule {
            kind:    TokenKind::Float,
            matches: |input| match_regex(input, &FLOAT_REGEX),
        },
        Rule {
            kind:    TokenKind::Identifier,
            matches: |input| match_regex(input, &IDENTIFIER_REGEX),
        },
    ]
}
