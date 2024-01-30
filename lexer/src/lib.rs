use token::{Span, Tag, Token};

#[derive(Default)]
pub struct Lexer {
    input: String,

    position: usize,
    read_positon: usize,
    ch: u8,
}

impl From<String> for Lexer {
    /// # Example
    /// ```
    /// use lexer::Lexer;
    ///
    /// let l = Lexer::from("int x = 12".to_string());
    /// let l: Lexer =  "float pi = 3.14;".to_string().into();
    /// ```
    fn from(input: String) -> Self {
        let mut l = Self {
            input,
            ..Default::default()
        };
        l.read_char();
        l
    }
}

impl From<&str> for Lexer {
    /// # Example
    /// ```
    /// use lexer::Lexer;
    ///
    /// let l = Lexer::from("int x = 12;");
    /// let l: Lexer = "float pi = 3.14;".into();
    /// ```
    fn from(input: &str) -> Self {
        Self::from(input.to_string())
    }
}

impl Lexer {
    /// Advances cursor in characterstream.
    #[inline]
    fn read_char(&mut self) {
        self.ch = *self.input.as_bytes().get(self.read_positon).unwrap_or(&0);
        self.position = self.read_positon;
        self.read_positon += 1;
    }

    /// Get the character which following the currently processing character.
    #[inline]
    fn peek_char(&self) -> u8 {
        *self.input.as_bytes().get(self.read_positon).unwrap_or(&0)
    }

    #[inline]
    fn read_integer(&mut self) -> usize {
        self.read_char(); // take the starting digit

        while self.ch >= b'0' && self.ch <= b'9' {
            self.read_char();
        }

        self.position
    }

    #[inline]
    fn is_letter(ch: u8) -> bool {
        match ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => true,
            _ => false,
        }
    }

    fn read_ident(&mut self) -> (&str, usize) {
        let pos = self.position; // store the current index
        self.read_char(); // take the starting identifier character

        while Self::is_letter(self.ch) {
            self.read_char();
        }

        (&self.input[pos..self.position], self.position)
    }

    pub fn next_token(&mut self) -> Token {
        let start = self.position;

        let tag = match self.ch {
            b'=' => Tag::ASSIGN,
            b'+' => {
                if self.peek_char() == b'+' {
                    self.read_char();
                    Tag::PLUS_PLUS
                } else {
                    Tag::PLUS
                }
            }

            b';' => Tag::SEMICOLON,
            b'(' => Tag::LPAREN,
            b')' => Tag::RPAREN,
            b'{' => Tag::LBRACE,
            b'}' => Tag::RBRACE,

            b'0'..=b'9' => {
                let end = self.read_integer();
                return Token {
                    tag: Tag::INTEGER,
                    span: Span(start, end),
                };
            }

            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let (literal, end) = self.read_ident();

                let tag = token::lookup_ident(literal);

                return Token {
                    tag,
                    span: Span(start, end),
                };
            }

            0 => {
                return Token {
                    tag: Tag::EOF,
                    span: Span(start, start),
                }
            }

            _ => Tag::ILLEGAL,
        };

        self.read_char();

        Token {
            tag,
            span: Span(start, self.position),
        }
    }
}
