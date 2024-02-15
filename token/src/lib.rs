mod tag;
pub use tag::Tag;

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
