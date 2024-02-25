use crate::{
    position::Position,
    token::{self, Token},
};

type ErrorHandle = dyn Fn(Position, String);

pub struct Scanner {
    filename: String,
    src: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,

    err: Box<ErrorHandle>,

    error_count: usize,
}

impl Scanner {
    pub fn new(filename: String, src: String, err: Box<ErrorHandle>) -> Self {
        let mut s = Self {
            filename,
            src,

            ch: b' ',
            offset: 0,
            rd_offset: 0,

            line_offset: 0,
            line_no: 1,

            err,
            error_count: 0,
        };

        s.next();
        s
    }

    fn error(&mut self, pos: Position, msg: String) {
        (self.err)(pos, msg);
        self.error_count += 1;
    }

    fn position(&self) -> Position {
        Position {
            filename: self.filename.clone(),
            offset: self.offset,
            line: self.line_no,
            column: self.offset - self.line_offset + 1,
        }
    }

    fn next(&mut self) {
        if let Some(&ch) = self.src.as_bytes().get(self.rd_offset) {
            self.offset = self.rd_offset;

            if self.ch == b'\n' {
                self.line_offset = self.offset;
                self.line_no += 1;
            }

            if ch == 0 {
                self.error(self.position(), format!("illegal character null"))
            }

            self.ch = ch;
            self.rd_offset += 1;
        } else {
            self.offset = self.src.len();
            self.ch = 0;

            if self.ch == b'\n' {
                self.line_offset = self.offset;
                self.line_no += 1;
            }
        }
    }

    fn advance(&mut self, len: usize) {
        if self.offset + len < self.src.len() {
            self.offset += len;
            self.ch = self.src.as_bytes()[self.offset];
        } else {
            self.offset = self.src.len();
            self.ch = 0;
        }
        self.rd_offset = self.offset + 1;
    }

    fn peek(&self) -> u8 {
        *self.src.as_bytes().get(self.rd_offset).unwrap_or(&0)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.next();
        }
    }

    fn scan_ident(&mut self) -> &str {
        let offset = self.offset;

        let len = count_till(&self.src[offset..], |c| {
            c.is_ascii_alphanumeric() || c == b'_' || c == b'$'
        });
        self.advance(len);

        &self.src[offset..offset + len]
    }

    fn switch(&mut self, def: Token, ch: u8, alt: Token) -> Token {
        if self.peek() == ch {
            self.next();
            alt
        } else {
            def
        }
    }

    fn switch2(&mut self, def: Token, alt: &[(u8, Token)]) -> Token {
        let peek = self.peek();

        for (c, t) in alt {
            if peek == *c {
                self.next();
                return *t;
            }
        }

        def
    }

    pub fn scan(&mut self) -> (Token, Position, &str) {
        self.skip_whitespace();

        let pos = self.position();

        if self.ch.is_ascii_alphabetic() || self.ch == b'_' || self.ch == b'$' {
            let lit = self.scan_ident();
            let tok = token::lookup(lit);
            return (tok, pos, lit);
        }

        if self.ch.is_ascii_digit() {
            while self.ch.is_ascii_digit() {
                self.next();
            }
            let lit = &self.src[pos.offset..self.offset];
            return (Token::INTEGER, pos, lit);
        }

        let tok = match self.ch {
            b'+' => self.switch2(
                Token::PLUS,
                &[(b'+', Token::INC), (b'=', Token::ADD_ASSIGN)],
            ),
            b'-' => self.switch2(
                Token::MINUS,
                &[
                    (b'-', Token::DEC),
                    (b'=', Token::SUB_ASSIGN),
                    (b'>', Token::ARROW),
                ],
            ),
            b'*' => self.switch(Token::ASTERISK, b'=', Token::MUL_ASSIGN),
            b'/' => self.switch(Token::SLASH, b'=', Token::DIV_ASSIGN),
            b'%' => self.switch(Token::REM, b'=', Token::REM_ASSIGN),

            b'&' => self.switch2(
                Token::AND,
                &[(b'&', Token::LAND), (b'=', Token::AND_ASSIGN)],
            ),

            b'|' => self.switch2(Token::OR, &[(b'|', Token::LOR), (b'=', Token::OR_ASSIGN)]),

            b'^' => self.switch(Token::XOR, b'=', Token::XOR_ASSIGN),

            b'<' => match self.peek() {
                b'=' => {
                    self.next();
                    Token::LEQ
                }
                b'<' => {
                    self.next();
                    if self.peek() == b'=' {
                        self.next();
                        Token::SHL_ASSIGN
                    } else {
                        Token::SHL
                    }
                }
                _ => Token::LT,
            },

            b'>' => match self.peek() {
                b'=' => {
                    self.next();
                    Token::GEQ
                }
                b'>' => {
                    self.next();
                    if self.peek() == b'=' {
                        self.next();
                        Token::SHR_ASSIGN
                    } else {
                        Token::SHR
                    }
                }
                _ => Token::GT,
            },

            b'~' => Token::TILDE,
            b'?' => Token::TERNARY,

            b'.' => {
                if self.peek() == b'.' {
                    self.next();

                    if self.peek() == b'.' {
                        self.next();
                        Token::ELIPSE
                    } else {
                        Token::ILLEGAL
                    }
                } else {
                    Token::DOT
                }
            }

            b'=' => self.switch(Token::ASSIGN, b'=', Token::EQ),
            b'!' => self.switch(Token::NOT, b'=', Token::NEQ),

            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b':' => Token::COLON,

            b'(' => Token::LPAREN,
            b'{' => Token::LBRACE,
            b'[' => Token::LBRACK,

            b')' => Token::RPAREN,
            b'}' => Token::RBRACE,
            b']' => Token::RBRACK,

            0 => return (Token::EOF, pos, ""),
            _ => Token::ILLEGAL,
        };

        self.next();

        let lit = &self.src[pos.offset..self.offset];

        (tok, pos, lit)
    }
}

fn count_till<T>(source: &str, cond: T) -> usize
where
    T: Fn(u8) -> bool,
{
    let mut len = 0;

    source
        .as_bytes()
        .iter()
        .take_while(|&&c| cond(c))
        .for_each(|_| len += 1);

    len
}

fn is_int_lit_ch(c: u8) -> bool {
    c >= b'0' && c <= b'9'
        || c >= b'a' && c <= b'f'
        || c >= b'A' && c <= b'F'
        || c == b'e'
        || c == b'E'
        || c == b'f'
        || c == b'F'
        || c == b'l'
        || c == b'L'
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    fn test_scan() {
        let tests = [
            (ILLEGAL, "@"),
            (IDENT, "intIs_32bit"),
            (IDENT, "_Give_me_100$"),
            (IDENT, "$"),
            (INTEGER, "1234567890"),
            // (INTEGER, "01234567"),
            // (INTEGER, "0x123456790abcdefABCDEF"),
            // (INTEGER, "0b1010"),
            // (FLOATING, "0."),
            // (FLOATING, ".1"),
            // (FLOATING, "3.1"),
            // (FLOATING, "9.e10"),
            // (FLOATING, "9.e-10"),
            // (FLOATING, "9.e+10"),
            // (FLOATING, "9.1e10"),
            // (FLOATING, "9.1e-10"),
            // (FLOATING, "9.1e+10"),
            // (FLOATING, ".1e10"),
            // (FLOATING, ".1e-10"),
            // (FLOATING, ".1e+10"),
            // (STRING, "\"crepl\""),
            // (STRING, "\"He said, \\\"I can eat 4 mango\\\".\""),
            (ASSIGN, "="),
            (ADD_ASSIGN, "+="),
            (SUB_ASSIGN, "-="),
            (MUL_ASSIGN, "*="),
            (DIV_ASSIGN, "/="),
            (REM_ASSIGN, "%="),
            (AND_ASSIGN, "&="),
            (OR_ASSIGN, "|="),
            (XOR_ASSIGN, "^="),
            (SHL_ASSIGN, "<<="),
            (SHR_ASSIGN, ">>="),
            (INC, "++"),
            (DEC, "--"),
            (PLUS, "+"),
            (MINUS, "-"),
            (ASTERISK, "*"),
            (SLASH, "/"),
            (REM, "%"),
            (TILDE, "~"),
            (AND, "&"),
            (OR, "|"),
            (XOR, "^"),
            (SHL, "<<"),
            (SHR, ">>"),
            (NOT, "!"),
            (LAND, "&&"),
            (LOR, "||"),
            (TERNARY, "?"),
            (DOT, "."),
            (ARROW, "->"),
            (ELIPSE, "..."),
            (COMMA, ","),
            (SEMICOLON, ";"),
            (COLON, ":"),
            (LPAREN, "("),
            (LBRACE, "{"),
            (LBRACK, "["),
            (RPAREN, ")"),
            (RBRACE, "}"),
            (RBRACK, "]"),
            (AUTO, "auto"),
            (BREAK, "break"),
            (CASE, "case"),
            (CHAR, "char"),
            (CONST, "const"),
            (CONTINUE, "continue"),
            (DEFAULT, "default"),
            (DO, "do"),
            (DOUBLE, "double"),
            (ELSE, "else"),
            (ENUM, "enum"),
            (EXTERN, "extern"),
            (FLOAT, "float"),
            (FOR, "for"),
            (GOTO, "goto"),
            (IF, "if"),
            (INLINE, "inline"),
            (INT, "int"),
            (LONG, "long"),
            (REGISTER, "register"),
            (RESTRICT, "restrict"),
            (RETURN, "return"),
            (SHORT, "short"),
            (SIGNED, "signed"),
            (SIZEOF, "sizeof"),
            (STATIC, "static"),
            (STRUCT, "struct"),
            (SWITCH, "switch"),
            (TYPEDEF, "typedef"),
            (UNION, "union"),
            (UNSIGNED, "unsigned"),
            (VOID, "void"),
            (VOLATILE, "volatile"),
            (WHILE, "while"),
        ];

        let source = tests
            .iter()
            .map(|(_, lit)| lit.to_owned())
            .collect::<Vec<_>>()
            .join(" ");

        let mut s = Scanner::new("repl".to_string(), source.clone(), Box::new(|_, _| {}));

        for (i, t) in tests.iter().enumerate() {
            let (tok, _, lit) = s.scan();

            assert_eq!(
                *t,
                (tok, lit),
                "source: {}\n [{}/{}] test failed.",
                source,
                i + 1,
                tests.len()
            );
        }
    }
}
