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
    SemiColon,
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
    Whitespace,
    Eof,
}

#[macro_export]
macro_rules! T {
    [+] => {
        $crate::lexer::TokenKind::Plus
    };
    [-] => {
        $crate::lexer::TokenKind::Minus
    };
    [*] => {
        $crate::lexer::TokenKind::Times
    };
    [/] => {
        $crate::lexer::TokenKind::Slash
    };
    [^] => {
        $crate::lexer::TokenKind::Pow
    };
    [=] => {
        $crate::lexer::TokenKind::Eq
    };
    [.] => {
        $crate::lexer::TokenKind::Dot
    };
    [,] => {
        $crate::lexer::TokenKind::Comma
    };
    [_] => {
        $crate::lexer::TokenKind::Underscore
    };
    [!] => {
        $crate::lexer::TokenKind::Bang
    };
    [&] => {
        $crate::lexer::TokenKind::Ampersand
    };
    [|] => {
        $crate::lexer::TokenKind::Bar
    };
    [:] => {
        $crate::lexer::TokenKind::Colon
    };
    [;] => {
        $crate::lexer::TokenKind::SemiColon
    };
    [<] => {
        $crate::lexer::TokenKind::LAngle
    };
    [>] => {
        $crate::lexer::TokenKind::RAngle
    };
    ['['] => {
        $crate::lexer::TokenKind::LSquare
    };
    [']'] => {
        $crate::lexer::TokenKind::RSquare
    };
    ['{'] => {
        $crate::lexer::TokenKind::LBrace
    };
    ['}'] => {
        $crate::lexer::TokenKind::RBrace
    };
    ['('] => {
        $crate::lexer::TokenKind::LParen
    };
    [')'] => {
        $crate::lexer::TokenKind::RParen
    };
    [string] => {
        $crate::lexer::TokenKind::String
    };
    [comment] => {
        $crate::lexer::TokenKind::Comment
    };
    [int] => {
        $crate::lexer::TokenKind::Int
    };
    [float] => {
        $crate::lexer::TokenKind::Float
    };
    [ident] => {
        $crate::lexer::TokenKind::Identifier
    };
    [let] => {
        $crate::lexer::TokenKind::KeywordLet
    };
    [fn] => {
        $crate::lexer::TokenKind::KeywordFn
    };
    [struct] => {
        $crate::lexer::TokenKind::KeywordStruct
    };
    [if] => {
        $crate::lexer::TokenKind::KeywordIf
    };
    [else] => {
        $crate::lexer::TokenKind::KeywordElse
    };
    [&&] => {
        $crate::lexer::TokenKind::And
    };
    [||] => {
        $crate::lexer::TokenKind::Or
    };
    [==] => {
        $crate::lexer::TokenKind::Eqq
    };
    [!=] => {
        $crate::lexer::TokenKind::Neq
    };
    [>=] => {
        $crate::lexer::TokenKind::Geq
    };
    [<=] => {
        $crate::lexer::TokenKind::Leq
    };
    [error] => {
        $crate::lexer::TokenKind::Error
    };
    [ws] => {
        $crate::lexer::TokenKind::Whitespace
    };
    [EOF] => {
        $crate::lexer::TokenKind::Eof
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
                T![;] => ";",
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
                T![ws] => "<WS>",
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
