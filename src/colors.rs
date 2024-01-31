use std::collections::HashMap;

use colorful::RGB;
use token::Tag;
use token::Tag::*;

#[allow(unused_variables)]
pub fn one_dark() -> HashMap<Tag, RGB> {
    let black = RGB::new(40, 44, 52);
    let red = RGB::new(224, 108, 117);
    let green = RGB::new(152, 195, 121);
    let yellow = RGB::new(229, 192, 123);
    let blue = RGB::new(97, 175, 239);
    let purple = RGB::new(198, 120, 221);
    let cyan = RGB::new(86, 182, 194);
    let gray = RGB::new(171, 178, 191);

    HashMap::from([
        (ILLEGAL, red),
        (EOF, RGB::new(0, 0, 0)),
        (IDENT, yellow),
        (INTEGER, yellow),
        (STRING, green),
        (ASSIGN, gray),
        (PLUS, gray),
        (INCREMENT, gray),
        (LESS_THAN, gray),
        (SEMICOLON, gray),
        (LPAREN, yellow),
        (RPAREN, yellow),
        (LBRACE, purple),
        (RBRACE, purple),
        (INT, purple),
        (FOR, purple),
    ])
}
