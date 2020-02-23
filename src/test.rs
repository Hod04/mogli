use crate::lexer;
use crate::token;
#[test]
fn next_token_test() {
  #[cfg_attr(rustfmt, rustfmt_skip)]
    let input = "
    let five = 5;
    let ten = 10;
    let add = fn(x, y) {
      x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
      return true;
    } else {
      return false;
    }
    10 == 10;
    10 != 9;
    ";

  let mut tests = Vec::new();

  tests.push(token::Token {
    token_type: token::TokenType::Let,
    literal: "let".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "five".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Assign,
    literal: "=".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "5".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Let,
    literal: "let".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "ten".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Assign,
    literal: "=".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "10".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Let,
    literal: "let".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "add".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Assign,
    literal: "=".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Function,
    literal: "fn".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LeftParenthesis,
    literal: "(".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "x".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Comma,
    literal: ",".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "y".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::RightParenthesis,
    literal: ")".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LeftBrace,
    literal: "{".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "x".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Plus,
    literal: "+".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "y".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::RightBrace,
    literal: "}".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Let,
    literal: "let".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "result".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Assign,
    literal: "=".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "add".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LeftParenthesis,
    literal: "(".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "five".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Comma,
    literal: ",".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Ident,
    literal: "ten".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::RightParenthesis,
    literal: ")".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Bang,
    literal: "!".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Minus,
    literal: "-".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Slash,
    literal: "/".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Asterisk,
    literal: "*".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "5".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "5".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LessThan,
    literal: "<".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "10".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::GreaterThan,
    literal: ">".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "5".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::If,
    literal: "if".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LeftParenthesis,
    literal: "(".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "5".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LessThan,
    literal: "<".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "10".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::RightParenthesis,
    literal: ")".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LeftBrace,
    literal: "{".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Return,
    literal: "return".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::True,
    literal: "true".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::RightBrace,
    literal: "}".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Else,
    literal: "else".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::LeftBrace,
    literal: "{".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Return,
    literal: "return".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::False,
    literal: "false".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::RightBrace,
    literal: "}".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "10".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Equal,
    literal: "==".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "10".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "10".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::NotEqual,
    literal: "!=".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Integer,
    literal: "9".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::Semicolon,
    literal: ";".to_string(),
  });
  tests.push(token::Token {
    token_type: token::TokenType::EOF,
    literal: "".to_string(),
  });

  let mut l = lexer::Lexer::new(input);
  for t in tests {
    let tok = l.next_token();
    assert_eq!(tok.token_type, t.token_type);
    assert_eq!(tok.literal, t.literal);
  }
}

#[test]
fn new_token_test() {
  let token = lexer::new_token(token::TokenType::EOF, 'c');
  assert_eq!(token.token_type, token::TokenType::EOF);
  assert_eq!(token.literal, "c");
}
