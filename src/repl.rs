use colorful::{Colorful, RGB};
use lexer::Lexer;
use rustyline::error::ReadlineError;
use token::Tag;

use crate::colors;

pub fn start() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    let pencil = colors::one_dark();

    loop {
        match rl.readline("> ") {
            Ok(input) => {
                let mut l = Lexer::from(input.clone());
                let mut t = l.next_token();

                while t.tag != Tag::EOF {
                    let tag_color = *pencil.get(&t.tag).unwrap_or(&RGB::new(171, 178, 191));
                    let literal = &input[t.span.0..t.span.1];

                    print!("{}", literal.color(tag_color));

                    t = l.next_token();
                }

                println!();
            }

            Err(e) => match e {
                ReadlineError::Eof => break,

                ReadlineError::Interrupted => {
                    match rl.readline(
                        "exiting?
- press ctrl-d to exit
- press ctrl-c again to exit
- press enter to contine",
                    ) {
                        Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
                        Err(e) => {
                            println!("repl error: {e}");
                            break;
                        }
                        _ => {}
                    }
                }
                _ => {
                    println!("repl error: {e}");
                    break;
                }
            },
        }
    }
}
