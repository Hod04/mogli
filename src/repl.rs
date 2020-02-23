use crate::lexer::Lexer;
use crate::token::TokenType;
use std::io::{self, stdout, BufRead, Write};

pub fn start() {
  let prompt = ">> ";
  let stdin = io::stdin();
  loop {
    print!("{}", prompt);
    stdout().flush().expect("Error flushing stdout");

    let mut line = String::new();
    stdin
      .lock()
      .read_line(&mut line)
      .expect("Error reading from stdin");
    let mut lexer = Lexer::new(&mut line);

    loop {
      let tok = lexer.next_token();
      println!("{:?}", tok);
      if tok.token_type == TokenType::EOF {
        break;
      }
    }
  }
}
