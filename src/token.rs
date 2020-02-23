use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
  Illegal,
  EOF,

  // Identifiers + literals
  Ident,
  Integer,

  // Operators
  Assign,
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  LessThan,
  GreaterThan,
  Equal,
  NotEqual,

  // Delimiters
  Comma,
  Semicolon,
  LeftParenthesis,
  RightParenthesis,
  LeftBrace,
  RightBrace,

  // Keywords
  Function,
  Let,
  True,
  False,
  If,
  Else,
  Return,
}

#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String,
}

pub fn lookup_indent(identifier: &str) -> TokenType {
  let keywords: HashMap<&str, TokenType> = [
    ("fn", TokenType::Function),
    ("let", TokenType::Let),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("if", TokenType::If),
    ("else", TokenType::Else),
    ("return", TokenType::Return),
  ]
  .iter()
  .cloned()
  .collect();

  match keywords.get(identifier) {
    Some(id) => *id,
    None => TokenType::Ident,
  }
}
