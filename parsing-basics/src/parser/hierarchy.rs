use super::{ast, Parser};
use crate::{lexer::Token, T};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn statement(&mut self) -> ast::Stmt {
        match self.peek() {
            T![let] => {
                self.consume(T![let]);
                let ident = self.next().expect("Expected identifier after `let`");
                assert_eq!(
                    ident.kind,
                    T![ident],
                    "Expected identifier after `let`, but found `{}`",
                    ident.kind
                );
                let name = self.text(ident).to_string();
                self.consume(T![=]);
                let value = self.expression();
                self.consume(T![;]);
                ast::Stmt::Let {
                    var_name: name,
                    value:    Box::new(value),
                }
            }
            T![ident] => {
                let ident = self.next().unwrap();
                let name = self.text(ident).to_string();
                self.consume(T![=]);
                let value = self.expression();
                self.consume(T![;]);
                ast::Stmt::Assignment {
                    var_name: name,
                    value:    Box::new(value),
                }
            }
            T![if] => {
                self.consume(T![if]);
                self.consume(T!['(']);
                let condition = self.expression();
                self.consume(T![')']);

                assert!(self.at(T!['{']), "Expected a block after `if` statement");
                let body = self.statement();
                let body = match body {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };
                self.consume(T!['}']);

                let else_stmt = if self.at(T![else]) {
                    self.consume(T![else]);
                    assert!(
                        self.at(T![if]) || self.at(T!['{']),
                        "Expected a block or an `if` after `else` statement"
                    );
                    Some(Box::new(self.statement()))
                } else {
                    None
                };

                ast::Stmt::IfStmt {
                    condition: Box::new(condition),
                    body,
                    else_stmt,
                }
            }
            T!['{'] => {
                self.consume(T!['{']);
                let mut stmts = Vec::new();
                while !self.at(T!['}']) {
                    let stmt = self.statement();
                    stmts.push(stmt);
                }
                ast::Stmt::Block { stmts }
            }
            kind => panic!("Unknown start of statement: `{}`", kind),
        }
    }
}
