use parsing_basics::{lexer::*, T};
use unindent::unindent;

/// walks `$tokens` and compares them to the given kinds.
macro_rules! assert_tokens {
    ($tokens:ident, [$($kind:expr,)*]) => {
        {
            let mut it = $tokens.iter();
            $(
                let token = it.next().expect("not enough tokens");
                assert_eq!(token.kind, $kind);
            )*
        }
    };
}

#[test]
fn single_char_tokens() {
    let input = "+-(.):";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    assert_tokens!(tokens, [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]);
}

#[test]
fn single_char_tokens_with_whitespace() {
    let input = "   + -  (.): ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let leading_space = &tokens[0];
    assert_eq!(leading_space.kind, T![ws]);
    assert_eq!(leading_space.len(), 3);

    let space_after_minus = &tokens[4];
    assert_eq!(space_after_minus.kind, T![ws]);
    assert_eq!(space_after_minus.len(), 2);

    let trailing_space = &tokens[9];
    assert_eq!(trailing_space.kind, T![ws]);
    assert_eq!(trailing_space.len(), 1);

    let tokens: Vec<_> = tokens.into_iter().filter(|t| t.kind != T![ws]).collect();
    assert_tokens!(tokens, [T![+], T![-], T!['('], T![.], T![')'], T![:], T![EOF],]);
}

#[test]
fn unknown_input() {
    let input = "{$$$$$$$+";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    assert_tokens!(tokens, [T!['{'], T![error], T![+], T![EOF],]);
}

#[test]
fn token_spans() {
    {
        let input = "+-(.):";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let dot = tokens[3];
        assert_eq!(dot.kind, T![.]);
        assert_eq!(dot.span, (3..4).into())
    }
    {
        let input = "{$$$$$$$+";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let error = tokens[1];
        assert_eq!(error.kind, T![error]);
        assert_eq!(error.span, (1..8).into())
    }
}

#[test]
fn maybe_multiple_char_tokens() {
    let input = "&&=<=_!=||";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    assert_tokens!(tokens, [T![&&], T![=], T![<=], T![_], T![!=], T![||], T![EOF],]);
}

#[test]
fn keywords() {
    let input = "if let = struct else fn";
    let mut lexer = Lexer::new(input);
    let tokens: Vec<_> = lexer.tokenize().into_iter().filter(|t| t.kind != T![ws]).collect();
    assert_tokens!(tokens, [T![if], T![let], T![=], T![struct], T![else], T![fn], T![EOF],]);
}

#[test]
#[rustfmt::skip]
fn function() {
    let input = r#"
        // tests stuff
        fn test(var: Type, var2_: bool) {
            let x = "String content \" test" + 7 / 27.3e-2^4;
            let chars = x.chars();
            if let Some(c) = chars.next() {
                x = x + c;
            } else if !var2_ {
                x = x + ",";
            }
        }
    "#;
    let input = unindent(input);
    let mut lexer = Lexer::new(input.as_str());
    let tokens: Vec<_> = lexer.tokenize().into_iter().filter(|t| t.kind != T![ws]).collect();
    assert_tokens!(tokens, [
        T![comment], // comment
        T![fn], T![ident], T!['('], T![ident], T![:], T![ident], T![,], T![ident], T![:], T![ident], T![')'], T!['{'], // function signature
            T![let], T![ident], T![=], T![string], T![+], T![int], T![/], T![float], T![^], T![int], T![;], // `x` assignment
            T![let], T![ident], T![=], T![ident], T![.], T![ident], T!['('], T![')'], T![;], // `chars` assignment
            T![if], T![let], T![ident], T!['('], T![ident], T![')'], T![=], T![ident], T![.], T![ident], T!['('], T![')'], T!['{'], // if
                T![ident], T![=], T![ident], T![+], T![ident], T![;], // `x` re-assignment
            T!['}'], T![else], T![if], T![!], T![ident], T!['{'], // else if
                T![ident], T![=], T![ident], T![+], T![string], T![;], // `x` re-assignment
            T!['}'], // end if
        T!['}'], // end fn
        T![EOF],
    ]);
}

#[test]
#[rustfmt::skip]
fn struct_def() {
    let input = r#"
        struct Foo<T> {
            bar: Bar<T>,
        }
    "#;
    let input = unindent(input);
    let input = input.as_str();
    let mut lexer = Lexer::new(input);
    let tokens: Vec<_> = lexer.tokenize().into_iter().filter(|t| t.kind != T![ws]).collect();
    assert_tokens!(tokens, [
        T![struct], T![ident], T![<], T![ident], T![>], T!['{'], // struct definition/type
            T![ident], T![:], T![ident], T![<], T![ident], T![>],T![,], // member `bar` of type `Bar<T>`
        T!['}'], // end struct
        T![EOF],
    ]);
    let bar = tokens[6];
    assert_eq!(bar.span, (20..23).into()); // unindented span
    assert_eq!(bar.text(input), "bar");

    let foo = tokens[1];
    assert_eq!(foo.text(input), "Foo");
}
