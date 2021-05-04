use parsing_basics::{
    lexer::*,
    parser::{ast, Parser},
    T,
};
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

#[test]
fn parse_expression() {
    fn parse(input: &str) -> ast::Expr {
        let mut parser = Parser::new(input);
        parser.expression()
    }

    // Weird spaces are to test that whitespace gets filtered out
    let expr = parse("42");
    assert_eq!(expr, ast::Expr::Literal(ast::Lit::Int(42)));
    let expr = parse("  2.7768");
    assert_eq!(expr, ast::Expr::Literal(ast::Lit::Float(2.7768)));
    let expr = parse(r#""I am a String!""#);
    assert_eq!(expr, ast::Expr::Literal(ast::Lit::Str("I am a String!".to_string())));
    let expr = parse("foo");
    assert_eq!(expr, ast::Expr::Ident("foo".to_string()));
    let expr = parse("bar (  x, 2)");
    assert_eq!(
        expr,
        ast::Expr::FnCall {
            fn_name: "bar".to_string(),
            args:    vec![ast::Expr::Ident("x".to_string()), ast::Expr::Literal(ast::Lit::Int(2)),],
        }
    );
    let expr = parse("!  is_visible");
    assert_eq!(
        expr,
        ast::Expr::PrefixOp {
            op:   T![!],
            expr: Box::new(ast::Expr::Ident("is_visible".to_string())),
        }
    );
    let expr = parse("(-13)");
    assert_eq!(
        expr,
        ast::Expr::PrefixOp {
            op:   T![-],
            expr: Box::new(ast::Expr::Literal(ast::Lit::Int(13))),
        }
    );
}

#[test]
fn parse_binary_expressions() {
    fn parse(input: &str) -> ast::Expr {
        let mut parser = Parser::new(input);
        parser.expression()
    }

    let expr = parse("4 + 2 * 3");
    assert_eq!(expr.to_string(), "(4 + (2 * 3))");

    let expr = parse("4 * 2 + 3");
    assert_eq!(expr.to_string(), "((4 * 2) + 3)");

    let expr = parse("4 - 2 - 3");
    assert_eq!(expr.to_string(), "((4 - 2) - 3)");

    let expr = parse("4 ^ 2 ^ 3");
    assert_eq!(expr.to_string(), "(4 ^ (2 ^ 3))");

    let expr = parse(r#"45.7 + 3 + 5 * 4^8^9 / 6 > 4 && test - 7 / 4 == "Hallo""#);
    assert_eq!(
        expr.to_string(),
        r#"((((45.7 + 3) + ((5 * (4 ^ (8 ^ 9))) / 6)) > 4) && ((test - (7 / 4)) == "Hallo"))"#
    );

    let expr = parse("2.0 / ((3.0 + 4.0) * (5.0 - 6.0)) * 7.0");
    assert_eq!(expr.to_string(), "((2 / ((3 + 4) * (5 - 6))) * 7)");

    let expr = parse("min ( test + 4 , sin(2*PI ))");
    assert_eq!(expr.to_string(), "min((test + 4),sin((2 * PI),),)");
}

#[test]
fn parse_postfix_op() {
    fn parse(input: &str) -> ast::Expr {
        let mut parser = Parser::new(input);
        parser.expression()
    }

    let expr = parse("4 + -2! * 3");
    assert_eq!(expr.to_string(), "(4 + ((- (2 !)) * 3))");
}

#[test]
fn parse_statements() {
    fn parse(input: &str) -> ast::Stmt {
        let mut parser = Parser::new(input);
        parser.statement()
    }

    let stmt = parse(
        unindent(
            r#"
        {
            let x = 7 + sin(y);
            {
                x = 3;
                if (bar < 3) {
                    x = x + 1;
                    y = 3 * x;
                } else if (bar < 2) {
                    let i = 2!;
                    x = x + i;
                } else {
                    x = 1;
                }
            }
        }
    "#,
        )
        .as_str(),
    );

    let stmts = match stmt {
        ast::Stmt::Block { stmts } => stmts,
        _ => unreachable!(),
    };
    assert_eq!(stmts.len(), 2);

    let let_stmt = &stmts[0];
    match let_stmt {
        ast::Stmt::Let { var_name, .. } => assert_eq!(var_name, "x"),
        _ => unreachable!(),
    }

    let stmts = match &stmts[1] {
        ast::Stmt::Block { stmts } => stmts,
        _ => unreachable!(),
    };
    assert_eq!(stmts.len(), 2);

    let assignment_stmt = &stmts[0];
    match assignment_stmt {
        ast::Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
        _ => unreachable!(),
    }

    let if_stmt = &stmts[1];
    match if_stmt {
        ast::Stmt::IfStmt {
            condition,
            body,
            else_stmt,
        } => {
            assert!(matches!(
                &**condition,
                ast::Expr::InfixOp {
                    op:  T![<],
                    lhs: _lhs,
                    rhs: _rhs,
                }
            ));
            assert_eq!(body.len(), 2);
            let x_assignment = &body[0];
            match x_assignment {
                ast::Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
                _ => unreachable!(),
            }
            let y_assignment = &body[1];
            match y_assignment {
                ast::Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "y"),
                _ => unreachable!(),
            }

            let else_stmt = match else_stmt {
                Some(stmt) => &**stmt,
                None => unreachable!(),
            };

            match else_stmt {
                ast::Stmt::IfStmt {
                    condition,
                    body,
                    else_stmt,
                } => {
                    assert!(matches!(
                        &**condition,
                        ast::Expr::InfixOp {
                            op:  T![<],
                            lhs: _lhs,
                            rhs: _rhs,
                        }
                    ));
                    assert_eq!(body.len(), 2);
                    let let_i = &body[0];
                    match let_i {
                        ast::Stmt::Let { var_name, .. } => assert_eq!(var_name, "i"),
                        _ => unreachable!(),
                    }
                    let x_assignment = &body[1];
                    match x_assignment {
                        ast::Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
                        _ => unreachable!(),
                    }

                    let else_stmt = match else_stmt {
                        Some(stmt) => &**stmt,
                        None => unreachable!(),
                    };

                    let stmts = match else_stmt {
                        ast::Stmt::Block { stmts } => stmts,
                        _ => unreachable!(),
                    };
                    assert_eq!(stmts.len(), 1);

                    let x_assignment = &stmts[0];
                    match x_assignment {
                        ast::Stmt::Assignment { var_name, .. } => assert_eq!(var_name, "x"),
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };
        }
        _ => unreachable!(),
    }
}

#[test]
fn parse_struct() {
    fn parse(input: &str) -> ast::Item {
        let mut parser = Parser::new(input);
        parser.item()
    }

    let item = parse(
        unindent(
            r#"
        struct Foo<T, U> {
            x: String,
            bar: Bar<Baz<T>, U>
        }
    "#,
        )
        .as_str(),
    );

    match item {
        ast::Item::Struct { name, members } => {
            assert_eq!(
                name,
                ast::Type {
                    name:     "Foo".to_string(),
                    generics: vec![
                        ast::Type {
                            name:     "T".to_string(),
                            generics: vec![],
                        },
                        ast::Type {
                            name:     "U".to_string(),
                            generics: vec![],
                        }
                    ],
                }
            );
            assert_eq!(members.len(), 2);
            let (bar, bar_type) = &members[1];
            assert_eq!(bar, "bar");
            assert_eq!(
                bar_type,
                &ast::Type {
                    name:     "Bar".to_string(),
                    generics: vec![
                        ast::Type {
                            name:     "Baz".to_string(),
                            generics: vec![ast::Type {
                                name:     "T".to_string(),
                                generics: vec![],
                            }],
                        },
                        ast::Type {
                            name:     "U".to_string(),
                            generics: vec![],
                        }
                    ],
                }
            );
        }
        _ => unreachable!(),
    };
}

#[test]
fn parse_function() {
    fn parse(input: &str) -> ast::Item {
        let mut parser = Parser::new(input);
        parser.item()
    }

    let item = parse(
        unindent(
            r#"
        fn wow_we_did_it(x: String, bar: Bar<Baz<T>, U>) {
            let x = 7 + sin(y);
            {
                x = 3;
                if (bar < 3) {
                    x = x + 1;
                    y = 3 * x;
                } else if (bar < 2) {
                    let i = 2!;
                    x = x + i;
                } else {
                    x = 1;
                }
            }
        }
    "#,
        )
        .as_str(),
    );

    match item {
        ast::Item::Function { name, parameters, body } => {
            assert_eq!(name, "wow_we_did_it");
            assert_eq!(parameters.len(), 2);
            let (bar, bar_type) = &parameters[1];
            assert_eq!(bar, "bar");
            assert_eq!(
                bar_type,
                &ast::Type {
                    name:     "Bar".to_string(),
                    generics: vec![
                        ast::Type {
                            name:     "Baz".to_string(),
                            generics: vec![ast::Type {
                                name:     "T".to_string(),
                                generics: vec![],
                            }],
                        },
                        ast::Type {
                            name:     "U".to_string(),
                            generics: vec![],
                        }
                    ],
                }
            );
            assert_eq!(body.len(), 2);
        }
        _ => unreachable!(),
    };
}
