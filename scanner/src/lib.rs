use token::Token;

pub struct Scanner {
    src: String,

    ch: u8,           // currently processing character
    offset: usize,    // position of the current character
    rd_offset: usize, // position of the next character

    lines: Vec<usize>, // offset of the lines
}

impl From<String> for Scanner {
    fn from(src: String) -> Self {
        let mut s = Self {
            src,
            ch: b' ',
            offset: 0,
            rd_offset: 0,
            lines: vec![0],
        };
        s.next();
        s
    }
}

impl Scanner {
    fn next(&mut self) {
        if let Some(&ch) = self.src.as_bytes().get(self.rd_offset) {
            self.offset = self.rd_offset;

            if self.ch == b'\n' {
                self.lines.push(self.offset);
            }

            self.ch = ch;
            self.rd_offset += 1;
        } else {
            self.offset = self.src.len();
            self.ch = 0;
        }
    }

    fn peek(&mut self) -> u8 {
        *self.src.as_bytes().get(self.rd_offset).unwrap_or(&0)
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.next();
        }
    }

    fn advance(&mut self, len: usize) {
        let offset = (self.offset + len).min(self.src.len());
        self.offset = offset;
        self.rd_offset = self.offset + 1;
        self.ch = self.src.as_bytes()[offset];
    }

    fn switch(&mut self, def: Token, alts: &[(u8, Token)]) -> Token {
        let p = self.peek();

        for &(c, t) in alts {
            if c == p {
                self.next();
                return t;
            }
        }

        def
    }

    pub fn scan(&mut self) -> Result<(Token, usize, &str), (Token, usize, &str, String)> {
        self.skip_whitespace();

        let pos = self.offset;

        let tok = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'$' => {
                let len = count_if(self.src[pos..].bytes(), is_letter);
                self.advance(len);
                let lit = &self.src[pos..pos + len];
                let tok = token::lookup(lit);

                return Ok((tok, pos, lit));
            }

            b'0' => {
                self.next();
                let prefix = self.ch;

                match prefix {
                    b'b' => {
                        self.next();

                        let digits = count_if(self.src[self.offset..].bytes(), is_binary_digit);
                        self.advance(digits);
                        let lit = &self.src[pos..self.offset];

                        if digits == 0 {
                            let msg = format!(
                                "at least one binary digit required after 0'{}'",
                                prefix as char
                            );
                            return Err((Token::ILLEGAL, pos, lit, msg));
                        }
                        return Ok((Token::INTEGER, pos, lit));
                    }

                    b'x' | b'X' => {
                        self.next();

                        let digits = count_if(self.src[self.offset..].bytes(), is_hex_digit);
                        self.advance(digits);
                        let lit = &self.src[pos..self.offset];

                        if digits == 0 {
                            let msg = format!(
                                "at least one hex digit required after 0'{}'",
                                prefix as char
                            );
                            return Err((Token::ILLEGAL, pos, lit, msg));
                        }
                        return Ok((Token::INTEGER, pos, lit));
                    }

                    _ => {
                        let digits = count_if(self.src[self.offset..].bytes(), is_octal_digit);
                        self.advance(digits);
                        let lit = &self.src[pos..self.offset];

                        return Ok((Token::INTEGER, pos, lit));
                    }
                }
            }

            b'1'..=b'9' => {
                let digits = count_if(self.src[self.offset..].bytes(), is_digit);
                self.advance(digits);
                let lit = &self.src[pos..self.offset];

                return Ok((Token::INTEGER, pos, lit));
            }

            b'+' => self.switch(
                Token::PLUS,
                &[(b'+', Token::INC), (b'=', Token::PLUS_ASSIGN)],
            ),

            b'-' => self.switch(
                Token::MINUS,
                &[
                    (b'-', Token::DEC),
                    (b'>', Token::ARROW),
                    (b'=', Token::MINUS_ASSIGN),
                ],
            ),

            b'*' => self.switch(Token::ASTERISK, &[(b'=', Token::MUL_ASSIGN)]),
            b'/' => self.switch(Token::SLASH, &[(b'=', Token::DIV_ASSIGN)]),
            b'%' => self.switch(Token::REM, &[(b'=', Token::REM_ASSIGN)]),

            b'&' => self.switch(
                Token::AND,
                &[(b'=', Token::AND_ASSIGN), (b'&', Token::LAND)],
            ),

            b'=' => self.switch(Token::ASSIGN, &[(b'=', Token::EQL)]),
            b'|' => self.switch(Token::OR, &[(b'=', Token::OR_ASSIGN)]),
            b'^' => self.switch(Token::XOR, &[(b'=', Token::XOR_ASSIGN)]),

            b';' => Token::SEMICOLON,

            0 => return Ok((Token::EOF, pos, "")),
            _ => {
                let ch = self.ch;

                self.next();

                let msg = format!("illegal character '{}' found", ch as char);

                return Err((Token::ILLEGAL, pos, &self.src[pos..self.offset], msg));
            }
        };

        self.next();

        Ok((tok, pos, &self.src[pos..self.offset]))
    }
}

fn is_letter(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b'$'
}

fn is_digit(c: u8) -> bool {
    c.is_ascii_digit()
}

fn is_binary_digit(c: u8) -> bool {
    c == b'0' || c == b'1'
}

fn is_hex_digit(c: u8) -> bool {
    c.is_ascii_hexdigit()
}

fn is_octal_digit(c: u8) -> bool {
    c >= b'0' && c <= b'7'
}

fn count_if<I: Copy>(src: impl Iterator<Item = I>, check: impl Fn(I) -> bool) -> usize {
    let mut count = 0;

    src.take_while(|i| check(*i)).for_each(|_| count += 1);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token::*;

    #[test]
    fn test_scan() {
        let tests = [
            // (ILLEGAL, "@"),
            (IDENT, "intIs_32bit"),
            (IDENT, "_Give_me_100$"),
            (IDENT, "$"),
            (INTEGER, "1234567890"),
            (INTEGER, "01234567"),
            (INTEGER, "0x123456790abcdefABCDEF"),
            (INTEGER, "0b1010"),
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

        let mut s = Scanner::from(source.clone());

        for (i, t) in tests.iter().enumerate() {
            let (tok, _, lit) = s.scan().unwrap();

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
