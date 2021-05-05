use super::TokenKind;
use crate::T;
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Eq)]
pub(super) enum LogosToken {
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(";")]
    Semi,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Slash,
    #[token("^")]
    Pow,
    #[token("=")]
    Eq,
    #[token("!")]
    Bang,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("==")]
    Eqq,
    #[token("!=")]
    Neq,
    #[token("<=")]
    Leq,
    #[token(">=")]
    Geq,
    #[token("_")]
    Under,
    // Brackets
    #[token("<")]
    LAngle,
    #[token(">")]
    RAngle,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LSquare,
    #[token("]")]
    RSquare,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    // Constructs
    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,
    #[regex(r#"//[^\n]*\n"#)]
    LineComment,
    #[regex(r#"\d+"#, priority = 2)]
    Int,
    #[regex(r#"((\d+(\.\d+)?)|(\.\d+))([Ee](\+|-)?\d+)?"#)]
    Float,
    #[regex(r#"[A-Za-z]([A-Za-z]|_|\d)*"#)]
    Ident,

    // Keywords
    #[token("let")]
    KwLet,
    #[token("if")]
    KwIf,
    #[token("else")]
    KwElse,
    #[token("fn")]
    KwFn,
    #[token("struct")]
    KwStruct,

    // Misc
    #[regex(r"[ \t\r\n\f]+")]
    WS,
    #[error]
    Error,
}

impl LogosToken {
    #[rustfmt::skip]
    pub fn kind(&self) -> TokenKind {
        use LogosToken::*;
        match self {
            Dot          => T![.],
            Colon        => T![:],
            Comma        => T![,],
            Semi         => T![;],
            Plus         => T![+],
            Minus        => T![-],
            Times        => T![*],
            Slash        => T![/],
            Pow          => T![^],
            Eq           => T![=],
            Bang         => T![!],
            And          => T![&&],
            Or           => T![||],
            Eqq          => T![==],
            Neq          => T![!=],
            Leq          => T![<=],
            Geq          => T![>=],
            Under        => T![_],
            LAngle       => T![<],
            RAngle       => T![>],
            LParen       => T!['('],
            RParen       => T![')'],
            LSquare      => T!['['],
            RSquare      => T![']'],
            LBrace       => T!['{'],
            RBrace       => T!['}'],
            String       => T![string],
            LineComment  => T![comment],
            Int          => T![int],
            Float        => T![float],
            Ident        => T![ident],
            KwLet        => T![let],
            KwIf         => T![if],
            KwElse       => T![else],
            KwFn         => T![fn],
            KwStruct     => T![struct],
            WS           => T![ws],
            Error        => T![error],
        }
    }
}
