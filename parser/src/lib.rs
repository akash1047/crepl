use lexer::Lexer;
use token::Token;

pub mod error;

pub struct Parser {
    l: Lexer,

    cur_token: Token,
    peek_token: Token,
}

impl From<Lexer> for Parser {
    /// Create Parser
    /// ```
    /// use parser::Parser;
    /// use lexer::Lexer;
    ///
    /// let l = Lexer::from("int x;");
    /// let p = Parser::from(l);
    /// ```
    fn from(mut l: Lexer) -> Self {
        let cur_token = l.next_token();
        let peek_token = l.next_token();
        Self {
            l,
            cur_token,
            peek_token,
        }
    }
}

impl Parser {}
