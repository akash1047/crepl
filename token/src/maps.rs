use crate::Token;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref TOKENS: HashMap<Token, &'static str> = HashMap::from([
        (Token::ILLEGAL, "ILLEGAL"),
        (Token::EOF, "EOF"),
        (Token::COMMENT, "COMMENT"),
        (Token::IDENT, "IDENT"),
        (Token::INTEGER, "INTEGER"),
        (Token::FLOATING, "FLOATING"),
        (Token::STRING, "STRING"),
        (Token::ASSIGN, "="),
        (Token::PLUS, "+"),
        (Token::MINUS, "-"),
        (Token::ASTERISK, "*"),
        (Token::SLASH, "/"),
        (Token::REM, "%"),
        (Token::BANG, "!"),
        (Token::TILDE, "~"),
        (Token::AND, "&"),
        (Token::OR, "|"),
        (Token::XOR, "^"),
        (Token::DOT, "."),
        (Token::TERNERY, "?"),
        (Token::INC, "++"),
        (Token::DEC, "--"),
        (Token::ARROW, "->"),
        (Token::LT, "<"),
        (Token::GT, ">"),
        (Token::LAND, "&&"),
        (Token::LOR, "||"),
        (Token::EQL, "=="),
        (Token::NEQ, "!="),
        (Token::LEQ, "<="),
        (Token::GEQ, ">="),
        (Token::PLUS_ASSIGN, "+="),
        (Token::MINUS_ASSIGN, "-="),
        (Token::MUL_ASSIGN, "*="),
        (Token::DIV_ASSIGN, "/="),
        (Token::REM_ASSIGN, "%="),
        (Token::AND_ASSIGN, "&="),
        (Token::OR_ASSIGN, "|="),
        (Token::XOR_ASSIGN, "^="),
        (Token::SHL_ASSIGN, "<<="),
        (Token::SHR_ASSIGN, ">>="),
        (Token::ELLIPSIS, "..."),
        (Token::LPAREN, "("),
        (Token::LBRACK, "["),
        (Token::LBRACE, "{"),
        (Token::COMMA, ","),
        (Token::RPAREN, ")"),
        (Token::RBRACK, "]"),
        (Token::RBRACE, "}"),
        (Token::SEMICOLON, ";"),
        (Token::COLON, ":"),
        (Token::AUTO, "auto"),
        (Token::BREAK, "break"),
        (Token::CASE, "case"),
        (Token::CHAR, "char"),
        (Token::CONST, "const"),
        (Token::CONTINUE, "continue"),
        (Token::DEFAULT, "default"),
        (Token::DO, "do"),
        (Token::DOUBLE, "double"),
        (Token::ELSE, "else"),
        (Token::ENUM, "enum"),
        (Token::EXTERN, "extern"),
        (Token::FLOAT, "float"),
        (Token::FOR, "for"),
        (Token::GOTO, "goto"),
        (Token::IF, "if"),
        (Token::INLINE, "inline"),
        (Token::INT, "int"),
        (Token::LONG, "long"),
        (Token::REGISTER, "register"),
        (Token::RESTRICT, "restrict"),
        (Token::RETURN, "return"),
        (Token::SHORT, "short"),
        (Token::SIGNED, "signed"),
        (Token::SIZEOF, "sizeof"),
        (Token::STATIC, "static"),
        (Token::STRUCT, "struct"),
        (Token::SWITCH, "switch"),
        (Token::TYPEDEF, "typedef"),
        (Token::UNION, "union"),
        (Token::UNSIGNED, "unsigned"),
        (Token::VOID, "void"),
        (Token::VOLATILE, "volatile"),
        (Token::WHILE, "while"),
        (Token::P_IF, "#if"),
        (Token::P_ELIF, "#elif"),
        (Token::P_ELSE, "#else"),
        (Token::P_ENDIF, "#endif"),
        (Token::P_IFDEF, "#ifdef"),
        (Token::P_IFNDEF, "#ifndef"),
        (Token::P_DEFINE, "#define"),
        (Token::P_UNDEF, "#undef"),
        (Token::P_INCLUDE, "#include"),
        (Token::P_LINE, "#line"),
        (Token::P_ERROR, "#error"),
        (Token::P_PRAGMA, "#pragma"),
        (Token::P_DEFINED, "#defined"),
    ]);
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, Token> = HashMap::from([
        ("auto", Token::AUTO),
        ("break", Token::BREAK),
        ("case", Token::CASE),
        ("char", Token::CHAR),
        ("const", Token::CONST),
        ("continue", Token::CONTINUE),
        ("default", Token::DEFAULT),
        ("do", Token::DO),
        ("double", Token::DOUBLE),
        ("else", Token::ELSE),
        ("enum", Token::ENUM),
        ("extern", Token::EXTERN),
        ("float", Token::FLOAT),
        ("for", Token::FOR),
        ("goto", Token::GOTO),
        ("if", Token::IF),
        ("inline", Token::INLINE),
        ("int", Token::INT),
        ("long", Token::LONG),
        ("register", Token::REGISTER),
        ("restrict", Token::RESTRICT),
        ("return", Token::RETURN),
        ("short", Token::SHORT),
        ("signed", Token::SIGNED),
        ("sizeof", Token::SIZEOF),
        ("static", Token::STATIC),
        ("struct", Token::STRUCT),
        ("switch", Token::SWITCH),
        ("typedef", Token::TYPEDEF),
        ("union", Token::UNION),
        ("unsigned", Token::UNSIGNED),
        ("void", Token::VOID),
        ("volatile", Token::VOLATILE),
        ("while", Token::WHILE),
    ]);
}
