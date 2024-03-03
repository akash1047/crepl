use token::{Position, Token};

pub struct Error {
    pub pos: Position,
    pub msg: String,
}

type ErrorHandle = Box<dyn Fn(Position, String)>;

pub struct Scanner {
    src: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,

    err: ErrorHandle,
}

impl Scanner {
    pub fn new(src: String, err: ErrorHandle) -> Self {
        let mut s = Self {
            src,
            ch: b' ',
            offset: 0,
            rd_offset: 0,
            line_offset: 0,
            line_no: 1,
            err,
        };
        s.next();
        s
    }

    fn next(&mut self) {
        if let Some(&ch) = self.src.as_bytes().get(self.rd_offset) {
            self.offset = self.rd_offset;
            if self.ch == b'\n' {
                self.line_offset = self.offset;
                self.line_no += 1;
            }

            if self.ch == 0 {
                self.error(format!("invalid character null"));
            }

            self.rd_offset += 1;
            self.ch = ch;
        } else {
            self.offset = self.src.len();
            if self.ch == b'\n' {
                self.line_offset = self.offset;
                self.line_no += 1;
            }
            self.ch = 0;
        }
    }
    fn error(&self, msg: String) {
        (self.err)(self.position(), msg);
    }

    fn peek(&self) -> u8 {
        *self.src.as_bytes().get(self.rd_offset).unwrap_or(&0)
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.next();
        }
    }

    #[inline]
    fn position(&self) -> Position {
        Position {
            offset: self.offset,
            line: self.line_no,
            column: self.offset - self.line_offset + 1,
        }
    }

    fn advance(&mut self, len: usize) {
        self.offset += len;
        self.rd_offset = self.offset + 1;
        self.ch = *self.src.as_bytes().get(self.offset).unwrap_or(&0);
    }

    pub fn switch(&mut self, def: Token, alt: &[(u8, Token)]) -> Token {
        let peek = self.peek();

        for (ch, tok) in alt {
            if peek == *ch {
                self.next();
                return *tok;
            }
        }

        def
    }
    fn expect(&mut self, ch: u8, tok: Token) -> Option<Token> {
        if self.peek() == ch {
            self.next();
            return Some(tok);
        }

        None
    }

    pub fn scan(&mut self) -> (Token, Position, &str) {
        self.skip_whitespace();

        let pos = self.position();

        if is_letter(self.ch) && !is_digit(self.ch) {
            while is_letter(self.ch) {
                self.next();
            }
            return (Token::IDENT, pos, &self.src[pos.offset..self.offset]);
        }

        if is_digit(self.ch) {
            while is_digit(self.ch) {
                self.next();
            }
            return (Token::INTEGER, pos, &self.src[pos.offset..self.offset]);
        }

        let tok = match self.ch {
            b'=' => Token::ASSIGN,

            b'+' => self.switch(
                Token::PLUS,
                &[(b'+', Token::INC), (b'=', Token::PLUS_ASSIGN)],
            ),

            b'-' => self.switch(
                Token::MINUS,
                &[
                    (b'-', Token::DEC),
                    (b'=', Token::MINUS_ASSIGN),
                    (b'>', Token::ARROW),
                ],
            ),

            b'<' => self.switch(Token::LT, &[(b'<', Token::SHL), (b'=', Token::LEQ)]),

            b'*' => self.switch(Token::ASTERISK, &[(b'=', Token::MUL_ASSIGN)]),

            0 => return (Token::EOF, pos, ""),
            _ => Token::ILLEGAL,
        };

        self.next();

        let lit = &self.src[pos.offset..self.offset];

        (tok, pos, lit)
    }
}

fn is_letter(c: u8) -> bool {
    c >= b'a' && c <= b'z'
        || c >= b'A' && c <= b'Z'
        || c >= b'0' && c <= b'9'
        || c == b'_'
        || c == b'$'
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}
fn is_hex_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9' || c >= b'A' && c <= b'F' || c >= b'a' && c <= b'f'
}
fn is_octal_digit(c: u8) -> bool {
    c >= b'0' && c <= b'7'
}
fn is_binary_digit(c: u8) -> bool {
    c == b'0' || c == b'1'
}

impl IntoIterator for Scanner {
    type Item = (Token, Position, String);

    type IntoIter = ScannerIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { scanner: self }
    }
}

pub struct ScannerIter {
    scanner: Scanner,
}

impl Iterator for ScannerIter {
    type Item = (Token, Position, String);

    fn next(&mut self) -> Option<Self::Item> {
        let (tok, pos, lit) = self.scanner.scan();

        if tok == Token::EOF {
            return None;
        }

        Some((tok, pos, lit.to_string()))
    }
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
            (PLUS_ASSIGN, "+="),
            (MINUS_ASSIGN, "-="),
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
            (TERNERY, "?"),
            (DOT, "."),
            (ARROW, "->"),
            (ELLIPSE, "..."),
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

        let mut s = Scanner::new(source.clone(), Box::new(|_, _| {}));

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
