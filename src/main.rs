use crate::{scanner::Scanner, token::Token};

mod position;
mod scanner;
mod token;

const PROMPT: &str = "> ";

const EXIT_PROMPT: &str = "trying to exit?
- press ctrl+c again to exit
- or press ctrl+d to exit";

fn main() {
    println!(
        "Hello {}, This is Monkey programming language!",
        whoami::realname()
    );

    let mut rl = rustyline::DefaultEditor::new().expect("Failed to create repl.");

    loop {
        match rl.readline(PROMPT) {
            Ok(input) => {
                let mut s = Scanner::new(
                    "C R e p L".to_string(),
                    input,
                    Box::new(|pos, msg| {
                        eprintln!(
                            "{msg}. line {}, column {}, {}",
                            pos.line, pos.column, pos.filename
                        )
                    }),
                );
                let (mut tok, _, mut lit) = s.scan();

                while tok != Token::EOF {
                    println!("({:?}, \"{lit}\")", tok);

                    (tok, _, lit) = s.scan();
                }
            }

            Err(e) => match e {
                rustyline::error::ReadlineError::Eof => break,
                rustyline::error::ReadlineError::Interrupted => match rl.readline(EXIT_PROMPT) {
                    Err(
                        rustyline::error::ReadlineError::Interrupted
                        | rustyline::error::ReadlineError::Eof,
                    ) => break,
                    Err(_) => break,
                    _ => {}
                },

                _ => break,
            },
        }
    }
}
