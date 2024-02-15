#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    ILLEGAL,
    EOF,
    COMMENT,

    IDENT,
    INTEGER,
    FLOATING,
    STRING,

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
}

impl Tag {
    pub fn to_str(&self) -> &str {
        match *self {
            Self::ILLEGAL => "ILLEGAL",
            Self::EOF => "EOF",
            Self::COMMENT => "COMMENT",

            Self::IDENT => "IDENT",
            Self::INTEGER => "INTEGER",
            Self::FLOATING => "FLOATING",
            Self::STRING => "STRING",

            Tag::ASSIGN => "=",
            Tag::PLUS => "+",
            Tag::MINUS => "-",
            Tag::ASTERISK => "*",
            Tag::SLASH => "/",
            Tag::REM => "%",
            Tag::BANG => "!",
            Tag::TILDE => "~",
            Tag::AND => "&",
            Tag::OR => "|",
            Tag::XOR => "^",
            Tag::DOT => ".",
            Tag::TERNERY => "?",

            Tag::INC => "++",
            Tag::DEC => "--",
            Tag::ARROW => "->",

            Tag::LT => "<",
            Tag::GT => ">",
            Tag::LAND => "&&",
            Tag::LOR => "||",
            Tag::EQL => "==",
            Tag::NEQ => "!=",
            Tag::LEQ => "<=",
            Tag::GEQ => ">=",

            Tag::PLUS_ASSIGN => "+=",
            Tag::MINUS_ASSIGN => "-=",
            Tag::MUL_ASSIGN => "*=",
            Tag::DIV_ASSIGN => "/=",
            Tag::REM_ASSIGN => "%=",
            Tag::AND_ASSIGN => "&=",
            Tag::OR_ASSIGN => "|=",
            Tag::XOR_ASSIGN => "^=",
            Tag::SHL_ASSIGN => "<<=",
            Tag::SHR_ASSIGN => ">>=",

            Tag::ELLIPSIS => "...",

            Tag::LPAREN => "(",
            Tag::LBRACK => "[",
            Tag::LBRACE => "{",
            Tag::COMMA => ",",

            Tag::RPAREN => ")",
            Tag::RBRACK => "]",
            Tag::RBRACE => "}",
            Tag::SEMICOLON => ";",
            Tag::COLON => ":",

            Tag::AUTO => "auto",
            Tag::BREAK => "break",
            Tag::CASE => "case",
            Tag::CHAR => "char",
            Tag::CONST => "const",
            Tag::CONTINUE => "continue",
            Tag::DEFAULT => "default",
            Tag::DO => "do",
            Tag::DOUBLE => "double",
            Tag::ELSE => "else",
            Tag::ENUM => "enum",
            Tag::EXTERN => "extern",
            Tag::FLOAT => "float",
            Tag::FOR => "for",
            Tag::GOTO => "goto",
            Tag::IF => "if",
            Tag::INLINE => "inline",
            Tag::INT => "int",
            Tag::LONG => "long",
            Tag::REGISTER => "register",
            Tag::RESTRICT => "restrict",
            Tag::RETURN => "return",
            Tag::SHORT => "short",
            Tag::SIGNED => "signed",
            Tag::SIZEOF => "sizeof",
            Tag::STATIC => "static",
            Tag::STRUCT => "struct",
            Tag::SWITCH => "switch",
            Tag::TYPEDEF => "typedef",
            Tag::UNION => "union",
            Tag::UNSIGNED => "unsigned",
            Tag::VOID => "void",
            Tag::VOLATILE => "volatile",
            Tag::WHILE => "while",

            Tag::P_IF => "#if",
            Tag::P_ELIF => "#elif",
            Tag::P_ELSE => "#else",
            Tag::P_ENDIF => "#endif",
            Tag::P_IFDEF => "#ifdef",
            Tag::P_IFNDEF => "#ifndef",
            Tag::P_DEFINE => "#define",
            Tag::P_UNDEF => "#undef",
            Tag::P_INCLUDE => "#include",
            Tag::P_LINE => "#line",
            Tag::P_ERROR => "#error",
            Tag::P_PRAGMA => "#pragma",
            Tag::P_DEFINED => "#defined",
        }
    }
}
