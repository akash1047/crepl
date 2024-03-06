#![allow(unused)]

// use app::App;
use scanner::Scanner;
use token::{Position, Token};

mod app;

fn main() {
    // let mut app = App::new();
    // app.start().unwrap();

    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let mut scanner = Scanner::from(line);

                loop {
                    match scanner.scan() {
                        Ok((Token::EOF, ..)) => break,

                        Ok((tok, _, lit)) => {
                            println!("({:?}, {})", tok, lit);
                        }

                        Err((_, pos, _, err)) => {
                            println!("Scanner Error: {}. at column {}.", err, pos + 1)
                        }
                    }
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}
