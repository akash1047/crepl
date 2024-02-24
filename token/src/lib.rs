mod maps;

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(C)]
pub enum Token {
    ILLEGAL,
    EOF,
    COMMENT,

    literal_beg,
    IDENT,
    INTEGER,
    BINARY,
    OCTAL,
    HEXADECIMAL,
    DECIMAL,
    FLOATING,
    STRING,
    literal_end,

    ASSIGN,   // =
    PLUS,     // +
    MINUS,    // -
    ASTERISK, // *
    SLASH,    // /
    REM,      // %
    BANG,     // !
    TILDE,    // ~
    AND,      // &
    OR,       // |
    XOR,      // ^
    DOT,      // .
    TERNERY,  // ?

    INC,   // ++
    DEC,   // --
    ARROW, // ->

    LT,   // <
    GT,   // >
    LAND, // &&
    LOR,  // ||
    EQL,  // ==
    NEQ,  // !=
    LEQ,  // <=
    GEQ,  // >=

    PLUS_ASSIGN,  // +=
    MINUS_ASSIGN, // -=
    MUL_ASSIGN,   // *=
    DIV_ASSIGN,   // /=
    REM_ASSIGN,   // %=
    AND_ASSIGN,   // &=
    OR_ASSIGN,    // |=
    XOR_ASSIGN,   // ^=
    SHL_ASSIGN,   // <<=
    SHR_ASSIGN,   // >>=

    ELLIPSIS, // ...

    LPAREN, // (
    LBRACK, // [
    LBRACE, // {
    COMMA,  // ,

    RPAREN,    // )
    RBRACK,    // ]
    RBRACE,    // }
    SEMICOLON, // ;
    COLON,     // :

    keyword_beg,
    AUTO,
    BREAK,
    CASE,
    CHAR,
    CONST,
    CONTINUE,
    DEFAULT,
    DO,
    DOUBLE,
    ELSE,
    ENUM,
    EXTERN,
    FLOAT,
    FOR,
    GOTO,
    IF,
    INLINE,
    INT,
    LONG,
    REGISTER,
    RESTRICT,
    RETURN,
    SHORT,
    SIGNED,
    SIZEOF,
    STATIC,
    STRUCT,
    SWITCH,
    TYPEDEF,
    UNION,
    UNSIGNED,
    VOID,
    VOLATILE,
    WHILE,
    keyword_end,

    preprocessor_beg,
    P_IF,
    P_ELIF,
    P_ELSE,
    P_ENDIF,
    P_IFDEF,
    P_IFNDEF,
    P_DEFINE,
    P_UNDEF,
    P_INCLUDE,
    P_LINE,
    P_ERROR,
    P_PRAGMA,
    P_DEFINED,
    preprocessor_end,
}

impl Token {
    pub fn to_str(&self) -> String {
        maps::TOKENS
            .get(self)
            .map_or_else(|| format!("Token({})", *self as i32), |&s| s.to_owned())
    }

    pub fn is_literal(&self) -> bool {
        *self > Self::literal_beg && *self < Self::literal_end
    }

    pub fn is_keyword(&self) -> bool {
        *self > Self::keyword_beg && *self < Self::keyword_end
    }

    pub fn is_preprocessor(&self) -> bool {
        *self > Self::preprocessor_beg && *self < Self::preprocessor_end
    }
}

pub fn lookup(ident: &str) -> Token {
    *maps::KEYWORDS.get(ident).unwrap_or(&Token::IDENT)
}

#[derive(Debug, Clone)]
pub struct Position {
    pub filename: String,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

pub type Literal = String;
