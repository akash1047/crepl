use token::{Literal, Position, Token};

type ErrorHandle = Box<dyn Fn(Position, String)>;

pub struct Scanner {
    filename: String,
    src: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,

    err: ErrorHandle,
}

impl Scanner {
    pub fn new(filename: String, src: String, err: ErrorHandle) -> Self {
        let mut s = Self {
            filename,
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
            filename: self.filename.clone(),
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

    #[inline]
    fn scan_ident(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'$')
                .map(|c| *c as char),
        )
    }

    #[inline]
    fn scan_integer(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c.is_ascii_digit())
                .map(|c| *c as char),
        )
    }
    /*  #[inline]
    fn scan_binary_integer(&self) -> Literal {
        String::from_iter(
            let len =2;
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c == b'0' || c == b'1')
                .for_each(|_| len+=1);


                .map(|c: &u8|  *c as char),
        )
    }*/
    /*  #[inline]
    fn scan_hexadecimal_integer(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c.is_ascii_hexdigit() )
                .map(|c: &u8|  *c as char),
        )
    }*/
    #[inline]
    fn scan_octal_integer(&self) -> Literal {
        String::from_iter(
            self.src.as_bytes()[self.offset..]
                .iter()
                .take_while(|&&c| c >= b'0' && c <= b'7')
                .map(|c: &u8| *c as char),
        )
    }

    pub fn scan(&mut self) -> (Token, Position, Literal) {
        self.skip_whitespace();

        let pos = self.position();

        if is_letter(self.ch) && !is_digit(self.ch) {
            let lit = self.scan_ident();
            self.advance(lit.len());

            let tok = token::lookup(lit.as_str());

            return (tok, pos, lit);
        }

        if is_digit(self.ch) {
            if self.ch == b'0' {
                match self.peek() {
                    b'b' => {
                        let offset = self.offset;

                        let mut len = 2;
                        self.src.as_bytes()[self.offset + 2..]
                            .iter()
                            .take_while(|&&c| c == b'0' || c == b'1')
                            .for_each(|_| len += 1);

                        let tok = if len == 2 {
                            self.error(format!("expcted at least one binary digit after '0b'"));
                            Token::ILLEGAL
                        } else {
                            Token::BINARY
                        };

                        self.advance(len);
                        return (tok, pos, self.src[offset..offset + len].to_string());
                    }

                    b'x' | b'X' => {
                        let offset = self.offset;
                        let mut len = 2;
                        self.src.as_bytes()[self.offset + 2..]
                            .iter()
                            .take_while(|&&c| c.is_ascii_hexdigit())
                            .for_each(|_| len += 1);

                        let tok = if len == 2 {
                            self.error(format!("expected at least one binary digit after 0x"));
                            Token::ILLEGAL
                        } else {
                            Token::BINARY
                        };

                        self.advance(len);

                        return (
                            Token::HEXADECIMAL,
                            pos,
                            self.src[offset..offset + len].to_string(),
                        );
                    }

                    b'0'..=b'7' => {
                        // octal integer litarl
                        let lit = self.scan_octal_integer();
                        return (Token::OCTAL, pos, lit);
                    }

                    _ => {}
                }
            }

            // else integer literal

            let lit = self.scan_integer();
            self.advance(lit.len());
            return (Token::DECIMAL, pos, lit);
        }

        let (tok, lit) = match self.ch {
            b'0'..=b'9' => {
                let lit = String::from_iter(
                    self.src.as_bytes()[self.offset..]
                        .iter()
                        .take_while(|&&c| c.is_ascii_digit())
                        .map(|c| *c as char),
                );

                self.offset += lit.len();
                self.rd_offset = self.offset + 1;
                self.ch = self.src.as_bytes()[self.offset];

                return (Token::INTEGER, pos.clone(), lit);
            }

            b';' => (Token::SEMICOLON, (self.ch as char).to_string()),
            0 => return (Token::EOF, pos, "".to_string()),
            _ => (Token::ILLEGAL, (self.ch as char).to_string()),
        };

        self.next();

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
    type Item = (Token, Position, Literal);

    type IntoIter = ScannerIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { scanner: self }
    }
}

pub struct ScannerIter {
    scanner: Scanner,
}

impl Iterator for ScannerIter {
    type Item = (Token, Position, Literal);

    fn next(&mut self) -> Option<Self::Item> {
        let token_info = self.scanner.scan();

        if token_info.0 == Token::EOF {
            return None;
        }

        Some(token_info)
    }
}
