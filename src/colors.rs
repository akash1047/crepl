use colorful::{core::color_string::CString, Colorful, ExtraColorInterface};
use token::Tag;

pub fn color_it(tag: Tag, literal: &str) -> CString {
    match tag {
        Tag::ILLEGAL => literal.red(),
        Tag::EOF => literal.grey0(),
        Tag::IDENT => literal.blue(),
        Tag::INTEGER => literal.yellow(),
        Tag::STRING => literal.green(),
        Tag::ASSIGN | Tag::PLUS | Tag::PLUS_PLUS => literal.magenta(),
        Tag::SEMICOLON => literal.dark_gray(),
        Tag::LPAREN | Tag::RPAREN => literal.magenta(),
        Tag::LBRACE | Tag::RBRACE => literal.light_gray(),
        _ => literal.magenta(),
    }
}
