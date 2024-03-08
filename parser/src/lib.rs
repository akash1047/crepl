use std::usize;

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

    fn expect2(&mut self, look_ahead: Token) -> Option<(Token, usize, String)> {
        if self.tok == look_ahead {
            let (tok, pos, lit) = (self.tok, self.pos, self.lit.clone());
            self.next();
            Some((tok, pos, lit))
        } else {
            // while (34535 == 4 { 2 = 12; }
            // expected ')' got 'INTEGER'
            let msg = if self.tok.is_literal() {
                format!("expected {}, got '{}'", look_ahead.to_str(), self.lit)
            } else {
                format!("expected {}, got {}", look_ahead.to_str(), self.tok.to_str())
            };

            self.errors.push((self.pos, msg));

            None
        }
    }

    fn parse_stmt(&mut self) -> Option<Box<dyn ast::Stmt>> {
        Some(match self.tok {
            Token::BREAK => {
                let pos = self.pos;
                self.next();

                if self.tok != Token::SEMICOLON {
                    return None;
                }

                let semi = self.pos;
                self.next();

                Box::new(ast::BreakStmt { pos, semi })
            }

            Token::RETURN => {
                let pos = self.pos;
                self.next();

                let value = self.parse_expr();

                if self.tok != Token::SEMICOLON {
                    return None;
                }
                let semi = self.pos;
                self.next();

                Box::new(ast::ReturnStmt { pos, value, semi })
            }

            Token::IF => {
                Box::new(self.parse_if_stmt()?)
            }

            Token::WHILE => {
                Box::new(self.parse_while_stmt()?)
            }

            Token::LBRACE => {
                let mut stmts = Vec::new();

                let lbrace = self.pos;
                self.next();

                // {
                //   break;
                //   return 0;
                //   {
                //     break;
                //     return ;
                //   }
                // }

                loop {
                    match self.parse_stmt() {
                        Some(s) => stmts.push(s),
                        None => break,
                    }
                }

                let rbrace = self.pos;
                self.next();

                Box::new(ast::BlockStmt {
                    lbrace,
                    stmts,
                    rbrace,
                })
            }

            _ => return None,
        })
    }

    fn parse_if_stmt(&mut self) -> Option<ast::IfStmt>{
        let if_pos: usize = self.pos;
        self.next();

    
        let (_, lparen_pos, _) = self.expect2(Token::LPAREN)?;

        let cond = self.parse_expr()?;

        let(_, rparen_pos, _) = self.expect2(Token::RPAREN)?;

        let init  = self.parse_stmt()?;

        let mut elifs: Vec<ast::ElseIf> = Vec::new();

        loop {
            if self.tok != Token::ELSE {
                break;
            }
            let else_pos = self.pos;

            if self.tok != Token::IF {
                break;
            }

            let if_pos: usize = self.pos;
            self.next();

    
            let (_, lparen_pos, _) = self.expect2(Token::LPAREN)?;

            let cond = self.parse_expr()?;

            let(_, rparen_pos, _) = self.expect2(Token::RPAREN)?;

            let init  = self.parse_stmt()?;

            elifs.push( ast::ElseIf {else_pos, if_pos, lparen_pos, cond, rparen_pos, init })
        }

        let _else = if self.tok == Token::ELSE {
            let pos = self.pos;
            let init = self.parse_stmt()?;

            Some(ast::Else { pos, init })
        } else { None };

        Some(ast::IfStmt {if_pos , lparen_pos , cond , rparen_pos , init, elifs, _else})

    }

    fn parse_while_stmt(&mut self) -> Option<ast::WhileStmt> {
        let pos = self.pos;
        self.next();

        let (_, lparen_pos, _) = self.expect2(Token::LPAREN)?;

        let cond = self.parse_expr()?;

        let (_, rparen_pos, _) = self.expect2(Token::RPAREN)?;

        let init = self.parse_stmt()?;

        Some(ast::WhileStmt { pos, lparen_pos, cond, rparen_pos, init })
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

            _ => None,
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

    #[test]
    fn test_stmt() {
        let source = "
break   ;
return


;

return 69         ;

";

        let tests = [
            "break;",
            "return;",
            "return 69;",
           
            // if (age >= 18) {
            //    if (age <= 21) {
            //        return 0;
            //    }
            //    return 1;
            // }
        ];

        let mut p = Parser::from(source.to_string());

        for (i, t) in tests.iter().enumerate() {
            let x = p.parse_stmt().unwrap();

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
