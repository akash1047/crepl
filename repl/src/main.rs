// use app::App;
use scanner::Scanner;
use token::Position;

mod app;

fn main() {
    // let mut app = App::new();
    // app.start().unwrap();

    let mut rl = rustyline::DefaultEditor::new().unwrap();

    let error_handle = |pos: Position, msg: String| {
        eprintln!(
            "Syntax Error: {msg}. line {}, column {}",
            pos.line, pos.column
        );
    };

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let scanner = Scanner::new(line.into(), Box::new(error_handle));

                for (tok, _, lit) in scanner.into_iter() {
                    if tok.is_literal() {
                        print!("{:?}({})", tok, lit);
                    } else {
                        print!("{:?}", tok);
                    }
                    println!();

                    // println!(" at line {}, column {}, {}", pos.line, pos.column, pos.filename);
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}
