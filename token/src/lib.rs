#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Tag {
    ILLEGAL,
    EOF,

    IDENT,
    INTEGER,
    STRING,

    ASSIGN,    // =
    PLUS,      // +
    PLUS_PLUS, // ++
    LESS_THAN, // <

    SEMICOLON, // ;
    LPAREN,    // (
    RPAREN,    // )
    LBRACE,    // {
    RBRACE,    // }

    INT,
    FOR,
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
        _ => Tag::IDENT,
    }
}
