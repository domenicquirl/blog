use super::{ast, Parser};
use crate::{lexer::Token, T};

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn file(&mut self) -> Vec<ast::Item> {
        let mut items = Vec::new();
        while !self.at(T![EOF]) {
            let item = self.item();
            items.push(item);
        }
        items
    }

    pub fn item(&mut self) -> ast::Item {
        match self.peek() {
            T![fn] => {
                self.consume(T![fn]);
                let mut parameters = Vec::new();

                let ident = self
                    .next()
                    .expect("Tried to parse function name, but there were no more tokens");
                assert_eq!(
                    ident.kind,
                    T![ident],
                    "Expected identifier as function name, but found `{}`",
                    ident.kind
                );
                let name = self.text(ident).to_string();

                self.consume(T!['(']);
                while !self.at(T![')']) {
                    let parameter_ident = self
                        .next()
                        .expect("Tried to parse function parameter, but there were no more tokens");
                    assert_eq!(
                        parameter_ident.kind,
                        T![ident],
                        "Expected identifier as function parameter, but found `{}`",
                        parameter_ident.kind
                    );
                    let parameter_name = self.text(parameter_ident).to_string();
                    self.consume(T![:]);
                    let parameter_type = self.type_();
                    parameters.push((parameter_name, parameter_type));
                    if self.at(T![,]) {
                        self.consume(T![,]);
                    }
                }
                self.consume(T![')']);

                assert!(self.at(T!['{']), "Expected a block after function header");
                let body = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };

                ast::Item::Function { name, parameters, body }
            }
            T![struct] => {
                self.consume(T![struct]);
                let mut members = Vec::new();
                let name = self.type_();
                self.consume(T!['{']);
                while !self.at(T!['}']) {
                    let member_ident = self
                        .next()
                        .expect("Tried to parse struct member, but there were no more tokens");
                    assert_eq!(
                        member_ident.kind,
                        T![ident],
                        "Expected identifier as struct member, but found `{}`",
                        member_ident.kind
                    );
                    let member_name = self.text(member_ident).to_string();
                    self.consume(T![:]);
                    let member_type = self.type_();
                    members.push((member_name, member_type));
                    if self.at(T![,]) {
                        self.consume(T![,]);
                    }
                }
                self.consume(T!['}']);
                ast::Item::Struct { name, members }
            }
            kind => panic!("Unknown start of item: `{}`", kind),
        }
    }

    pub fn type_(&mut self) -> ast::Type {
        let ident = self.next().expect("Tried to parse type, but there were no more tokens");
        assert_eq!(
            ident.kind,
            T![ident],
            "Expected identifier at start of type, but found `{}`",
            ident.kind
        );
        let name = self.text(ident).to_string();

        let mut generics = Vec::new();

        if self.at(T![<]) {
            self.consume(T![<]);
            while !self.at(T![>]) {
                // Generic parameters are also types
                let generic = self.type_();
                generics.push(generic);
                if self.at(T![,]) {
                    self.consume(T![,]);
                }
            }
            self.consume(T![>]);
        }

        ast::Type { name, generics }
    }

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
                self.consume(T!['}']);
                ast::Stmt::Block { stmts }
            }
            kind => panic!("Unknown start of statement: `{}`", kind),
        }
    }
}
