use crate::token;
use crate::token::{lookup_indent, Token, TokenType};
use std::collections::HashSet;

pub struct Lexer<'a> {
  input: &'a str,
  position: usize,
  read_position: usize,
  current_char: Option<char>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &str) -> Lexer {
    let mut l = Lexer {
      input,
      position: 0,
      read_position: 0,
      current_char: None,
    };
    l.read_char();
    l
  }

  pub fn next_token(&mut self) -> Token {
    let mut tok = token::Token {
      token_type: token::TokenType::Illegal,
      literal: String::new(),
    };

    self.skip_whitespace();

    match self.current_char {
      Some(ch @ '=') => {
        if self.peek_char() == Some('=') {
          self.read_char();
          tok = Token {
            token_type: TokenType::Equal,
            literal: [
              char_to_string(ch),
              char_to_string(self.current_char.unwrap()),
            ]
            .concat(),
          }
        } else {
          tok = new_token(TokenType::Assign, ch);
        }
      }
      Some(ch @ '!') => {
        if self.peek_char() == Some('=') {
          self.read_char();
          tok = Token {
            token_type: TokenType::NotEqual,
            literal: [
              char_to_string(ch),
              char_to_string(self.current_char.unwrap()),
            ]
            .concat(),
          }
        } else {
          tok = new_token(TokenType::Bang, ch)
        }
      }
      Some(ch @ ';') => tok = new_token(TokenType::Semicolon, ch),
      Some(ch @ '(') => tok = new_token(TokenType::LeftParenthesis, ch),
      Some(ch @ ')') => tok = new_token(TokenType::RightParenthesis, ch),
      Some(ch @ ',') => tok = new_token(TokenType::Comma, ch),
      Some(ch @ '{') => tok = new_token(TokenType::LeftBrace, ch),
      Some(ch @ '}') => tok = new_token(TokenType::RightBrace, ch),
      Some(ch @ '+') => tok = new_token(TokenType::Plus, ch),
      Some(ch @ '-') => tok = new_token(TokenType::Minus, ch),
      Some(ch @ '/') => tok = new_token(TokenType::Slash, ch),
      Some(ch @ '*') => tok = new_token(TokenType::Asterisk, ch),
      Some(ch @ '<') => tok = new_token(TokenType::LessThan, ch),
      Some(ch @ '>') => tok = new_token(TokenType::GreaterThan, ch),
      Some(ch @ _) => {
        if is_letter(ch) {
          tok.literal = self.read_identifier();
          tok.token_type = lookup_indent(&tok.literal);
          return tok;
        } else if is_digit(ch) {
          tok.literal = self.read_number();
          tok.token_type = TokenType::Integer;
          return tok;
        } else {
          tok = new_token(TokenType::Illegal, ch)
        }
      }
      None => {
        tok.token_type = TokenType::EOF;
        tok.literal = String::new();
      }
    }

    self.read_char();
    tok
  }

  pub fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.current_char = None;
    } else {
      self.current_char = self.input.chars().nth(self.read_position);
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn peek_char(&mut self) -> Option<char> {
    if self.read_position >= self.input.len() {
      return None;
    } else {
      return self.input.chars().nth(self.read_position);
    }
  }

  fn read_identifier(&mut self) -> String {
    let position = self.position;
    while is_letter(self.current_char.unwrap()) {
      self.read_char();
    }
    self.input[position..self.position].to_owned().to_string()
  }

  fn read_number(&mut self) -> String {
    let position = self.position;
    while is_digit(self.current_char.unwrap()) {
      self.read_char();
    }
    self.input[position..self.position].to_owned().to_string()
  }

  fn skip_whitespace(&mut self) {
    let skip_map: HashSet<char> = [(' '), ('\t'), ('\n'), ('\r')].iter().cloned().collect();
    loop {
      match self.current_char {
        Some(ch) => {
          if skip_map.contains(&ch) {
            self.read_char();
          } else {
            return;
          }
        }
        None => return,
      }
    }
  }
}

fn is_letter(ch: char) -> bool {
  'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
  '0' <= ch && ch <= '9'
}

pub fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
  token::Token {
    token_type: token_type,
    literal: char_to_string(ch),
  }
}

fn char_to_string(ch: char) -> String {
  ch.to_owned().to_string()
}
