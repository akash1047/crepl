use token::{Literal, Position, Token};

#[derive(Debug)]
pub struct Scanner {
    filename: String,
    src: String,

    ch: u8,
    offset: usize,
    rd_offset: usize,

    line_offset: usize,
    line_no: usize,
}

impl Scanner {
    pub fn new(filename: String, src: String) -> Self {
        let mut s = Self {
            filename,
            src,
            ch: b' ',
            offset: 0,
            rd_offset: 0,
            line_offset: 0,
            line_no: 1,
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

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.next();
        }
    }

    fn position(&self) -> Position {
        Position {
            filename: self.filename.clone(),
            offset: self.offset,
            line: self.line_no,
            column: self.offset - self.line_offset + 1,
        }
    }

    pub fn scan(&mut self) -> (Token, Position, Literal) {
        self.skip_whitespace();

        let pos = self.position();

        let (tok, lit) = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'$' => {
                let lit = String::from_iter(
                    self.src.as_bytes()[self.offset..]
                        .iter()
                        .take_while(|&&c| c.is_ascii_alphanumeric() || c == b'_' || c == b'$')
                        .map(|c| *c as char),
                );

                self.offset += lit.len();
                self.rd_offset = self.offset + 1;
                self.ch = self.src.as_bytes()[self.offset];

                let tok = token::lookup(lit.as_str());

                return (tok, pos, lit);
            }

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

                return (Token::INTEGER, pos, lit);
            }

            b';' => (Token::SEMICOLON, (self.ch as char).to_string()),
            0 => return (Token::EOF, pos, "".to_string()),
            _ => (Token::ILLEGAL, (self.ch as char).to_string()),
        };

        self.next();

        (tok, pos, lit)
    }
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
