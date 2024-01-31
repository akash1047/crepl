#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    ILLEGAL,
    EOF,

    IDENT,
    INTEGER,
    STRING,
    FLOATING,
    CHARACTER,
    //OPERATORS
    PLUS,        // +
    MINUS,       // -
    STAR,        // *
    SLASH,       // /
    MODULO,      //%
    LESS_THAN,   // <
    GREATER_THAN, // >

    LESS_THAN_EQUAL, // <=
    GREATER_THAN_EQUAL,  // >=
    NOT_EQUAL,     // !=
    EQUALITY,     // ==
    AND,          // &&
    OR,           // ||
    LOGICAL_NOT,  // !
    BITWISE_AND,  // &
    BITWISE_OR,  // |
    BITWISE_XOR, //^
    SHIFT_LEFT,  // <<
    SHIFT_RIGHT, // >>
    BITWISE_NOT, //~
    DECREMENT, // --
    POINTER, //  ->*
    //ADDRESS, //&




    ASSIGN,    // =
    PLUS_PLUS, // ++
   //DELIMITERS
    SEMICOLON, // ;
    COMMA,     //,
    COLON,     // :
    DOT,       //.
    LBRACKET,  //[
    RBRACKET,  //]
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }
  //keywords
    INT,
    FOR,
    AUTO,
    DO,
    IF,
    ELSE,
    RETURN,
    SIZEOF,
    WHILE,
    CHAR,
    CONST,
    SHORT,
    VOLATILE,
    DOUBLE,
    FLOAT,
    LONG,
    SIGNED,
    UNSIGNED,
    STATIC,
    STRUCT,
    UNION,
    ENUM,
    CASE,
    GOTO,
    SWITCH,
    BREAK,
    DEFAULT,
    REGISTER,
    CONTINUE,
    //COMPOUND ASSIGN
    PLUS_ASSIGN, //+= D
    MINUS_ASSIGN,   //-=D
    DIV_ASSIGN,  ///=
    MOD_ASSIGN,  // %
    PROD_ASSIGN, // *=
    POW, //** 
    LEFT_SHIFT_ASSIGN, // <<=
    RIGHT_SHIFT_ASSIGN,// >>=
    AND_ASSIGN,  //&=
    XOR_ASSIGN,  //^=
    OR_ASSIGN,   //|=

}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub tag: Tag,
    pub span: Span,
}

pub fn lookup_ident(ident: &str) -> Tag {
    match ident {
        "int" => Tag::INT,
        "for" => Tag::FOR,
        "auto" => Tag::AUTO,
        "do" => Tag::DO,
        "if" => Tag::IF,
        "else" => Tag::ELSE,
        "return" => Tag::RETURN,
        "sizeof" => Tag::SIZEOF,
        "while" => Tag::WHILE,
        "char" => Tag::CHAR,
        "const" => Tag::CONST,
        "short" => Tag::SHORT,
        "volatile" => Tag::VOLATILE,
        "double" => Tag::DOUBLE,
        "float" => Tag::FLOAT,
        "long" => Tag::LONG,
        "signed" => Tag::SIGNED,
        "unsigned" => Tag::UNSIGNED,
        "static" => Tag::STATIC,
        "struct" => Tag::STRUCT,
        "union" => Tag::UNION,
        "enum" => Tag::ENUM,
        "case" => Tag::CASE,
        "goto" => Tag::GOTO,
        "switch" => Tag::SWITCH,
        "default" => Tag::DEFAULT,
        "register" => Tag::REGISTER,
        "break" => Tag::BREAK,
        "continue" => Tag::CONTINUE,
        _ => Tag::IDENT,
    }
}
