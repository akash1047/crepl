use std::error::Error;

use scanner::Scanner;
use token::{Position, Token};

pub struct Parser {
    scanner: Scanner,

    // The next token
    tok: Token,
    pos: Position,
    lit: String,

    errors: Vec<scanner::Error>,
}

impl<T: AsRef<str>> From<T> for Parser {
    fn from(source: T) -> Self {
        todo!()
    }
}

impl Parser {
    fn next(&mut self) {
        let (tok, pos, lit) = self.scanner.scan();

        self.tok = tok;
        self.pos = pos;
        self.lit = lit.to_string();
    }

    fn expect(&mut self, peek: Token) -> bool {
        if peek == self.tok {
            self.next();
            true
        } else {
            let msg = if self.tok.is_literal() {
                format!("expected '{:?}' got '{}'", peek, &self.lit)
            } else {
                format!("expected '{:?}' got '{:?}'", peek, self.tok)
            };

            let err = scanner::Error { pos: self.pos, msg };

            self.errors.push(err);

            false
        }
    }

    fn expect2(&mut self, peek: Token) -> Option<(Token, Position, String)> {
        if peek == self.tok {
            let t = (self.tok, self.pos, self.lit.clone());
            self.next();
            Some(t)
        } else {
            let msg = if self.tok.is_literal() {
                format!("expected '{:?}' got '{}'", peek, &self.lit)
            } else {
                format!("expected '{:?}' got '{:?}'", peek, self.tok)
            };

            let err = scanner::Error { pos: self.pos, msg };

            self.errors.push(err);
            None
        }
    }

    fn parse_program(&mut self) -> Vec<Box<dyn ast::Stmt>> {
        let mut stmts = Vec::new();

        while self.tok != Token::EOF {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
        }

        stmts
    }

    fn parse_stmt(&mut self) -> Option<Box<dyn ast::Stmt>> {
        match self.tok {
            Token::BREAK => Some(Box::new(self.parse_break_stmt()?)),
            Token::CONTINUE => Some(Box::new(self.parse_continue_stmt()?)),

            _ => todo!(""),
        }
    }

    fn parse_break_stmt(&mut self) -> Option<ast::BreakStmt> {
        let stmt = ast::BreakStmt { pos: self.pos };
        self.next();

        if !self.expect(Token::SEMICOLON) {
            return None;
        }

        Some(stmt)
    }

    fn parse_continue_stmt(&mut self) -> Option<ast::ContinueStmt> {
        let stmt = ast::ContinueStmt { pos: self.pos };
        self.next();

        if !self.expect(Token::SEMICOLON) {
            return None;
        }

        Some(stmt)
    }
}
