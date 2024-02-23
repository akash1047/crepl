use scanner::Scanner;

fn main() {
    let mut rl = rustyline::DefaultEditor::new().unwrap();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let scanner = Scanner::new("program.repl".into(), line.into());

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
