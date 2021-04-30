use std::{
    fmt,
    ops::{Index, Range},
};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        (self.span.end - self.span.start) as usize
    }

    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} - <{}, {}>", self.kind, self.span.start, self.span.end)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Default, Debug)]
pub struct Span {
    /// inclusive
    pub start: u32,
    /// exclusive
    pub end:   u32,
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start as usize..span.end as usize
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end:   range.end as u32,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenKind {
    // Single characters
    Plus,
    Minus,
    Times,
    Slash,
    Pow,
    Eq,
    Dot,
    Comma,
    Underscore,
    Bang,
    Ampersand,
    Bar,
    Colon,
    // Brackets
    LAngle,
    RAngle,
    LSquare,
    RSquare,
    LBrace,
    RBrace,
    LParen,
    RParen,
    // Multiple characters
    String,
    Comment,
    Int,
    Float,
    Identifier,
    KeywordLet,
    KeywordFn,
    KeywordStruct,
    KeywordIf,
    KeywordElse,
    // Operators
    And,
    Or,
    Eqq,
    Neq,
    Geq,
    Leq,
    // Misc,
    Error,
    Eof,
}

#[macro_export]
macro_rules! T {
    [+] => {
        $crate::lexer::token::TokenKind::Plus
    };
    [-] => {
        $crate::lexer::token::TokenKind::Minus
    };
    [*] => {
        $crate::lexer::token::TokenKind::Times
    };
    [/] => {
        $crate::lexer::token::TokenKind::Slash
    };
    [^] => {
        $crate::lexer::token::TokenKind::Pow
    };
    [=] => {
        $crate::lexer::token::TokenKind::Eq
    };
    [.] => {
        $crate::lexer::token::TokenKind::Dot
    };
    [,] => {
        $crate::lexer::token::TokenKind::Comma
    };
    [_] => {
        $crate::lexer::token::TokenKind::Underscore
    };
    [!] => {
        $crate::lexer::token::TokenKind::Bang
    };
    [&] => {
        $crate::lexer::token::TokenKind::Ampersand
    };
    [|] => {
        $crate::lexer::token::TokenKind::Bar
    };
    [:] => {
        $crate::lexer::token::TokenKind::Colon
    };
    [<] => {
        $crate::lexer::token::TokenKind::LAngle
    };
    [>] => {
        $crate::lexer::token::TokenKind::RAngle
    };
    ['['] => {
        $crate::lexer::token::TokenKind::LSquare
    };
    [']'] => {
        $crate::lexer::token::TokenKind::RSquare
    };
    ['{'] => {
        $crate::lexer::token::TokenKind::LBrace
    };
    ['}'] => {
        $crate::lexer::token::TokenKind::RBrace
    };
    ['('] => {
        $crate::lexer::token::TokenKind::LParen
    };
    [')'] => {
        $crate::lexer::token::TokenKind::RParen
    };
    [string] => {
        $crate::lexer::token::TokenKind::String
    };
    [comment] => {
        $crate::lexer::token::TokenKind::Comment
    };
    [int] => {
        $crate::lexer::token::TokenKind::Int
    };
    [float] => {
        $crate::lexer::token::TokenKind::Float
    };
    [ident] => {
        $crate::lexer::token::TokenKind::Identifier
    };
    [let] => {
        $crate::lexer::token::TokenKind::KeywordLet
    };
    [fn] => {
        $crate::lexer::token::TokenKind::KeywordFn
    };
    [struct] => {
        $crate::lexer::token::TokenKind::KeywordStruct
    };
    [if] => {
        $crate::lexer::token::TokenKind::KeywordIf
    };
    [else] => {
        $crate::lexer::token::TokenKind::KeywordElse
    };
    [&&] => {
        $crate::lexer::token::TokenKind::And
    };
    [||] => {
        $crate::lexer::token::TokenKind::Or
    };
    [==] => {
        $crate::lexer::token::TokenKind::Eqq
    };
    [!=] => {
        $crate::lexer::token::TokenKind::Neq
    };
    [>=] => {
        $crate::lexer::token::TokenKind::Geq
    };
    [<=] => {
        $crate::lexer::token::TokenKind::Leq
    };
    [error] => {
        $crate::lexer::token::TokenKind::Error
    };
    [EOF] => {
        $crate::lexer::token::TokenKind::Eof
    };
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Single characters
                T![+] => "+",
                T![-] => "-",
                T![*] => "*",
                T![/] => "/",
                T![^] => "^",
                T![=] => "=",
                T![.] => ".",
                T![,] => ",",
                T![_] => "_",
                T![!] => "!",
                T![&] => "&",
                T![|] => "|",
                T![:] => ":",
                // Brackets
                T![<] => "<",
                T![>] => ">",
                T!['['] => "[",
                T![']'] => "]",
                T!['{'] => "{",
                T!['}'] => "}",
                T!['('] => "(",
                T![')'] => ")",
                // Multiple characters
                T![string] => "String",
                T![comment] => "// Comment",
                T![int] => "Int",
                T![float] => "Float",
                T![ident] => "Identifier",
                T![let] => "let",
                T![fn] => "fn",
                T![struct] => "struct",
                T![if] => "if",
                T![else] => "else",
                // Operators
                T![&&] => "&&",
                T![||] => "||",
                T![==] => "==",
                T![!=] => "!=",
                T![>=] => ">=",
                T![<=] => "<=",
                // Misc
                T![error] => "<?>",
                T![EOF] => "<EOF>",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_kind_display() {
        assert_eq!(T![+].to_string(), "+");
        assert_eq!(T![<=].to_string(), "<=");
        assert_eq!(T![let].to_string(), "let");
        assert_eq!(T![error].to_string(), "<?>");
        assert_eq!(T![comment].to_string(), "// Comment");
    }
}
