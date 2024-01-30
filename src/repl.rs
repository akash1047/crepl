use lexer::Lexer;
use rustyline::error::ReadlineError;
use token::Tag;

use crate::colors::color_it;

pub fn start() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        match rl.readline("> ") {
            Ok(input) => {
                let mut l = Lexer::from(input.clone());
                let mut t = l.next_token();

                while t.tag != Tag::EOF {
                    print!("{}", color_it(t.tag, &input[t.span.0..t.span.1]));
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
