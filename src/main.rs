pub mod lexer;
pub mod repl;
pub mod test;
pub mod token;

fn main() {
    print!("Hello! This is the Mogli programming language!\n");
    repl::start()
}
