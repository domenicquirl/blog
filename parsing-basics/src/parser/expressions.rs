use crate::{lexer::Token, T};

use super::{ast, Parser};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_expression(&mut self) -> ast::Expr {
        match self.peek() {
            lit @ T![int] | lit @ T![float] | lit @ T![string] => {
                let literal_text = {
                    // if `peek` is not `T![EOF]`, then there must be a next token
                    let literal_token = self.next().unwrap();
                    self.text(literal_token)
                };
                let lit = match lit {
                    T![int] => ast::Lit::Int(
                        literal_text
                            .parse()
                            .expect(&format!("invalid integer literal: `{}`", literal_text)),
                    ),
                    T![float] => ast::Lit::Float(
                        literal_text
                            .parse()
                            .expect(&format!("invalid floating point literal: `{}`", literal_text)),
                    ),
                    T![string] => ast::Lit::Str(literal_text[1..(literal_text.len() - 1)].to_string()),
                    _ => unreachable!(),
                };
                ast::Expr::Literal(lit)
            }
            T![ident] => {
                let name = {
                    let ident_token = self.next().unwrap();
                    self.text(ident_token).to_string()
                };
                if !self.at(T!['(']) {
                    // plain identifier
                    ast::Expr::Ident(name)
                } else {
                    //  function call
                    let mut args = Vec::new();
                    self.consume(T!['(']);
                    while !self.at(T![')']) {
                        let arg = self.parse_expression();
                        args.push(arg);
                        if self.at(T![,]) {
                            self.consume(T![,]);
                        }
                    }
                    self.consume(T![')']);
                    ast::Expr::FnCall { fn_name: name, args }
                }
            }
            T!['('] => {
                // There is no AST node for grouped expressions.
                // Parentheses just influence the tree structure.
                self.consume(T!['(']);
                let expr = self.parse_expression();
                self.consume(T![')']);
                expr
            }
            op @ T![+] | op @ T![-] | op @ T![!] => {
                self.consume(op);
                let expr = self.parse_expression();
                ast::Expr::PrefixOp {
                    op,
                    expr: Box::new(expr),
                }
            }
            kind => {
                panic!("Unknown start of expression: `{}`", kind);
            }
        }
    }
}
