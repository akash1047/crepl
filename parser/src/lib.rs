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
    // - - 4     + . -3;
    // (-(-4)) + (-3)

    fn parse_expr(&mut self) -> Option<Box<dyn ast::Expr>> {
        let x = self.parse_operand()?;
        match self.tok {
            Token::PLUS
            | Token::MINUS
            | Token::ASTERISK
            | Token::SLASH
            | Token::REM
            | Token::AND
            | Token::OR
            | Token::XOR
            | Token::SHL
            | Token::SHR
            | Token::LAND
            | Token::LOR
            | Token::EQL
            | Token::NEQ
            | Token::LT
            | Token::GT
            | Token::LEQ
            | Token::GEQ => {
                //parsing infix operation
                let op_pos = self.pos;
                let op = self.tok;

                self.next();

                let y = self.parse_expr()?;

                Some(Box::new(ast::InfixExpr { x, op_pos, op, y }))
            }

            _ => Some(x),
        }
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
            Token::IDENT => {
                let pos = self.pos;
                let name = self.lit.clone();

                self.next();

                Some(Box::new(ast::Ident { pos, name }))
            }

            Token::PLUS | Token::MINUS => {
                let op_pos = self.pos;
                let op = self.tok;

                self.next();

                let x = self.parse_operand()?;

                Some(Box::new(ast::UnaryExpr { op_pos, op, x }))
            }

            Token::INTEGER | Token::FLOATING | Token::STRING => {
                let pos = self.pos;
                let tok = self.tok;
                let lit = self.lit.clone();

                self.next();

                Some(Box::new(ast::BasicLit { pos, tok, lit }))
            }

            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_expr() {
        let source = "-1 +2 + x 12 - - - 3";

        // (((-1) + 2) + x)
        // (12 - (-(-3)))
        //(12/6)
        // let tests = ["(-1)", "(+2)", "x", "12", "(-(-(-3)))"];
        let tests = ["((-1) + (2 + x))", "(12 - (-(-3)))"];

        let mut p = Parser::from(source.to_string());

        for (i, t) in tests.iter().enumerate() {
            let x = p.parse_expr().unwrap();

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

#[test]
fn test_parse_expr1() {
    let source = "12 / 3";
    //(12 / 3)
    let tests = ["(12 / 3)"];

    let mut p = Parser::from(source.to_string());

    for (i, t) in tests.iter().enumerate() {
        let x = p.parse_expr().unwrap();

        assert_eq!(
            *t,
            x.string(),
            "[{}/{}] test case failed.",
            i + 1,
            tests.len()
        );
    }
}
