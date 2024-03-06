use scanner::Scanner;
use token::Token;

#[derive(Default, Debug)]
pub struct Parser {
    scanner: Scanner,

    tok: Token,
    pos: usize,
    lit: String,

    errors: Vec<(usize, String)>,
}

impl From<String> for Parser {
    fn from(src: String) -> Self {
        Self::from(Scanner::from(src))
    }
}

impl From<Scanner> for Parser {
    fn from(scanner: Scanner) -> Self {
        let mut p = Self {
            scanner,
            ..Self::default()
        };
        p.next();
        p
    }
}

impl Parser {
    fn next(&mut self) {
        loop {
            match self.scanner.scan() {
                Ok((tok, pos, lit)) => {
                    self.tok = tok;
                    self.pos = pos;
                    self.lit = lit.to_string();

                    break;
                }
                Err((_, pos, _, msg)) => {
                    self.errors.push((pos, msg));
                }
            }
        }
    }

    fn expect(&mut self, t: Token) {
        if self.tok == t {
            self.next();
        } else {
            let msg = if self.tok.is_literal() {
                format!("expected {}, got '{}'", t.to_str(), self.lit)
            } else {
                format!("expected {}, got {}", t.to_str(), self.tok.to_str())
            };

            self.errors.push((self.pos, msg));
        }
    }

    fn parse_break_stmt(&mut self) -> ast::BreakStmt {
        let pos = self.pos;
        self.expect(Token::SEMICOLON);
        ast::BreakStmt { pos }
    }

    fn parse_continue_stmt(&mut self) -> ast::ContinueStmt {
        let pos = self.pos;
        self.expect(Token::SEMICOLON);
        ast::ContinueStmt { pos }
    }

    fn parse_ident(&mut self) -> ast::Ident {
        let pos = self.pos;
        let name = self.lit.clone();

        self.next();

        ast::Ident { pos, name }
    }

    fn parse_expr(&mut self) -> Option<Box<dyn ast::Expr>> {
        None
    }

    fn parse_basic_lit(&mut self) -> ast::BasicLit {
        let tok = self.tok;
        let pos = self.pos;
        let lit = self.lit.clone();

        self.next();

        ast::BasicLit { pos, tok, lit }
    }

    fn parse_operand(&mut self) -> Option<Box<dyn ast::Expr>> {
        match self.tok {
            Token::IDENT => Some(Box::new(self.parse_ident())),

            Token::PLUS | Token::MINUS => {
                eprintln!("PARSING UNARY EXPRESSION");
                let op_pos = self.pos;
                let op = self.tok;

                self.next();

                let x = self.parse_operand()?;

                Some(Box::new(ast::UnaryExpr { op_pos, op, x }))
            }

            Token::INTEGER | Token::FLOATING | Token::STRING => {
                Some(Box::new(self.parse_basic_lit()))
            }

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operand() {
        let source = "-1 +2 x 12 - - - 3";

        let tests = ["(-1)", "(+2)", "x", "12", "(-(-(-3)))"];

        let mut p = Parser::from(source.to_string());

        for (i, t) in tests.iter().enumerate() {
            let x = p.parse_operand().unwrap();

            assert_eq!(
                *t,
                x.string(),
                "[{}/{}] test case failed.",
                i + 1,
                tests.len()
            );
        }
    }
}
