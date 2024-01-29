fn main() {
    // println!("Hello, world!");
    greeting("This is crepl an interpreter for C!");
}
fn greeting(msg:&str){
    

println!("Hi {}! {}", whoami::realname().split(" ").collect::<Vec<_>>()[0], msg);
}
