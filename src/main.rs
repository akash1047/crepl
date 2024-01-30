mod repl;
mod colors;

fn main() {
    greeting("This is crepl an interpreter for C!");

    repl::start();
}

fn greeting(msg: &str) {
    println!(
        "Hi {}! {}",
        whoami::realname().split(" ").collect::<Vec<_>>()[0],
        msg
    );
}
