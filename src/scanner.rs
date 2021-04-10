// use errors::{ErrorKind, Result};
// use literal::{Literal, Number};
use crate::token::{Item, Token};
// use crate::errors::LoxError;
// use token_type::TokenType;

pub struct Scanner {
    characters: Vec<char>,
    current: usize,
    start: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        let source: String = source.into();
        let characters = source.chars().collect::<Vec<_>>();
        Scanner {
            characters,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Item> {
        let mut tokens = vec![];
        while self.check_done_scanning() {
            self.start = self.current;
            let token = self.scan_token().expect("Whoopsie");
            if let Some(token) = token {
                tokens.push(token);
            } 
        }
        tokens
    }

    fn scan_token(&mut self) -> Result<Option<Item>, String> {
        let character = self.characters[self.current];
        self.current += 1;
        Ok(match character {
            '(' => Some(Item::new(character.to_string(), Token::LeftParen, self.line)),
            ')' => Some(Item::new(character.to_string(), Token::RightParen, self.line)),
            '{' => Some(Item::new(character.to_string(), Token::LeftBrace, self.line)),
            '}' => Some(Item::new(character.to_string(), Token::RightBrace, self.line)),
            ',' => Some(Item::new(character.to_string(), Token::Comma, self.line)),
            '.' => Some(Item::new(character.to_string(), Token::Dot, self.line)),
            '-' => Some(Item::new(character.to_string(), Token::Minus, self.line)),
            '+' => Some(Item::new(character.to_string(), Token::Plus, self.line)),
            ';' => Some(Item::new(character.to_string(), Token::SemiColon, self.line)),
            '*' => Some(Item::new(character.to_string(), Token::Star, self.line)),

            '=' => {
                Some(if self.match_char('=') {
                    self.make_token(Token::EqualEqual)
                } else {
                    self.make_token(Token::Equal)
                })
            },

            '!' => {
                Some(if self.match_char('=') {
                    self.make_token(Token::BangEqual)
                } else {
                    self.make_token(Token::Bang)
                })
            },

            '<' => {
                Some(if self.match_char('=') {
                    self.make_token(Token::LesserEqual)
                } else {
                    self.make_token(Token::Lesser)
                })
            },
            '>' => {
                Some(if self.match_char('=') {
                    self.make_token(Token::GreaterEqual)
                } else {
                    self.make_token(Token::Greater)
                })
            },

            '/' => {
                if self.match_char('/') {
                    while self.characters[self.current] != '\n' && self.check_done_scanning() {
                        self.current += 1;
                    }
                    None
                } else {
                    Some(self.make_token(Token::Slash))
                }
            },

            '"' => Some(self.scan_string()?),

            ' ' | '\r' | '\t' => {
                None
            },

            digit if self.is_digit(digit) => {
                Some(self.scan_number()?)
            },

            alpha if self.is_alpha(alpha) => {
                self.scan_ident()
            },

            _ => {
                return Err(format!(
                    "No matching token '{}' on line {}",
                    character, self.line
                ))
            }
            // LoxError {
            //     line: self.line,
            //     message: String::from("no matching token")
            // }
        })
    }

    fn scan_string(&mut self) -> Result<Item, String> {
        while self.characters[self.current] != '"' && self.check_done_scanning() {
            if self.characters[self.current] == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if !self.check_done_scanning() {
            return Err(format!("Unterminated string literal at line {}", self.line));
        }

        // the closing '"'
        self.current += 1;

        let start = self.start + 1;
        let end = self.current - 1;
        let val = self.characters[start..end].iter().cloned().collect::<String>();
        Ok(self.make_token(Token::String(val)))
    }

    fn scan_number(&mut self) -> Result<Item, String> {
        let mut is_float = false;
        while self.is_digit(self.characters[self.current]) {
            self.current += 1;
        }

        if self.characters[self.current] == '.' && self.is_digit(self.characters[self.current + 1]) {
            is_float = true;
            // Consume the '.'
            self.current += 1;

            while self.is_digit(self.characters[self.current]) {
                self.current += 1;
            }
        }

        let lexeme = self.characters[self.start..self.current].iter().cloned().collect::<String>();
        Ok(self.make_token(Token::Number(if is_float {
            let parsed: f64 = lexeme.parse().expect("Failed parsing to float");
            // let literal = Number::Float(parsed);
            parsed
        } else {
            let parsed: i64 = lexeme.parse().expect("Failed parsing to int");
            // let literal = Number::Int(parsed);
            parsed as f64
        })))
    }

    fn scan_ident(&mut self) -> Option<Item> {
        while self.is_alphanumeric(self.characters[self.current]) {
            self.current += 1;
        }
        use Token::*;
        // let text = self.characters[self.start..self.current].iter().cloned().collect();
        let mut text = "".to_owned();
        for char in self.characters[self.start..self.current].iter() {
            text.push_str(&char.to_string());
        }

        let token = match &*text {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "fun" => Fun,
            "for" => For,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Identifier(text),
        };

        Some(self.make_token(token))
    }

    fn is_digit(&self, ch: char) -> bool {
        ch >= '0' && ch <= '9'
    }

    fn is_alpha(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        self.is_alpha(ch) || self.is_digit(ch)
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.check_done_scanning() {
            return false;
        }
        if self.characters[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn make_token(&self, token: Token) -> Item {
        let lexeme = self.characters[self.start..self.current]
            .iter()
            .cloned()
            .collect::<String>();
        Item::new(lexeme, token, self.line)
    }

    fn check_done_scanning(&self) -> bool {
        self.current < self.characters.len()
    }
}

// impl Scanner {
//     pub fn new(source: &str) -> Scanner {
//         let source: String = source.into();
//         let chars = source.chars().collect::<Vec<_>>();
//         Scanner {
//             characters: chars,
//             start: 0,
//             current: 0,
//             line: 1,
//         }
//     }

//     fn is_at_end(&self) -> bool {
//         self.current >= self.characters.len()
//     }

//     pub fn scan(mut self) -> Result<Vec<Token>> {
//         let mut tokens = vec![];

//         while !self.is_at_end() {
//             self.start = self.current;
//             let token = self.scan_tokens()?;
//             if let Some(token) = token {
//                 tokens.push(token);
//             }
//         }

//         tokens.push(Token::new(TokenType::Eof, "", None, self.line));
//         Ok(tokens)
//     }

//     fn scan_tokens(&mut self) -> Result<Option<Token>> {
//         let ch = self.next();
//         Ok(match ch {
//             // Single-char tokens
//             '(' => Some(self.make_token(TokenType::LeftParen, None)),
//             ')' => Some(self.make_token(TokenType::RightParen, None)),
//             '{' => Some(self.make_token(TokenType::LeftBrace, None)),
//             '}' => Some(self.make_token(TokenType::RightBrace, None)),
//             ',' => Some(self.make_token(TokenType::Comma, None)),
//             '.' => Some(self.make_token(TokenType::Dot, None)),
//             '-' => Some(self.make_token(TokenType::Minus, None)),
//             '+' => Some(self.make_token(TokenType::Plus, None)),
//             ';' => Some(self.make_token(TokenType::Semicolon, None)),
//             '*' => Some(self.make_token(TokenType::Star, None)),

//             // Single-or-double-char tokens
//             '!' => {
//                 if self.match_char('=') {
//                     Some(self.make_token(TokenType::BangEqual, None))
//                 } else {
//                     Some(self.make_token(TokenType::Bang, None))
//                 }
//             }
//             '=' => {
//                 if self.match_char('=') {
//                     Some(self.make_token(TokenType::EqualEqual, None))
//                 } else {
//                     Some(self.make_token(TokenType::Equal, None))
//                 }
//             }
//             '<' => {
//                 if self.match_char('=') {
//                     Some(self.make_token(TokenType::LesserEqual, None))
//                 } else {
//                     Some(self.make_token(TokenType::Lesser, None))
//                 }
//             }
//             '>' => {
//                 if self.match_char('=') {
//                     Some(self.make_token(TokenType::GreaterEqual, None))
//                 } else {
//                     Some(self.make_token(TokenType::Greater, None))
//                 }
//             }

//             // Could be single-char token, or a comment
//             '/' => {
//                 if self.match_char('/') {
//                     while self.peek() != '\n' && !self.is_at_end() {
//                         self.next();
//                     }
//                     None
//                 } else {
//                     Some(self.make_token(TokenType::Slash, None))
//                 }
//             }

//             // Ignore Whitespace
//             ' ' | '\r' | '\t' => None,

//             '\n' => {
//                 self.line += 1;
//                 self.scan_tokens()?
//             }

//             '"' => Some(self.scan_string()?),

//             digit if self.is_digit(digit) => Some(self.scan_number()?),

//             alpha if self.is_alpha(alpha) => Some(self.scan_ident()?),

//             _ => return Err(ErrorKind::LoxError(self.line, "no matched token".into()).into()),
//         })
//     }

//     fn is_digit(&self, ch: char) -> bool {
//         ch >= '0' && ch <= '9'
//     }

//     fn is_alpha(&self, ch: char) -> bool {
//         (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
//     }

//     fn is_alphanumeric(&self, ch: char) -> bool {
//         self.is_alpha(ch) || self.is_digit(ch)
//     }

//     fn scan_ident(&mut self) -> Result<Token> {
//         while self.is_alphanumeric(self.peek()) {
//             self.next();
//         }

//         let text = self.characters[self.start..self.current]
//             .iter()
//             .cloned()
//             .collect::<String>();
//         if let Some(ttype) = TokenType::keyword(&text) {
//             Ok(self.make_token(ttype, None))
//         } else {
//             Ok(self.make_token(TokenType::Identifier, None))
//         }
//     }

//     fn scan_number(&mut self) -> Result<Token> {
//         let mut is_float = false;
//         while self.is_digit(self.peek()) {
//             self.next();
//         }

//         if self.peek() == '.' && self.is_digit(self.peek_next()) {
//             is_float = true;
//             // Consume the '.'
//             self.next();

//             while self.is_digit(self.peek()) {
//                 self.next();
//             }
//         }

//         let lexeme = self.characters[self.start..self.current]
//             .iter()
//             .cloned()
//             .collect::<String>();
//         Ok(self.make_token(
//             TokenType::Number,
//             if is_float {
//                 let parsed: f64 = lexeme.parse()?;
//                 let literal = Number::Float(parsed);
//                 Some(Literal::Number(literal))
//             } else {
//                 let parsed: i64 = lexeme.parse()?;
//                 let literal = Number::Int(parsed);
//                 Some(Literal::Number(literal))
//             },
//         ))
//     }

//     fn scan_string(&mut self) -> Result<Token> {
//         while self.peek() != '"' && !self.is_at_end() {
//             if self.peek() == '\n' {
//                 self.line += 1;
//             }
//             self.next();
//         }

//         if self.is_at_end() {
//             return Err(ErrorKind::LoxError(self.line, "unterminated string!".into()).into());
//         }

//         // the closing '"'
//         self.next();

//         let start = self.start + 1;
//         let end = self.current - 1;
//         let val = self.characters[start..end]
//             .iter()
//             .cloned()
//             .collect::<String>();
//         let lit = Literal::String(val);
//         Ok(self.make_token(TokenType::String, Some(lit)))
//     }

//     fn peek(&self) -> char {
//         if self.is_at_end() {
//             '\0'
//         } else {
//             self.characters[self.current]
//         }
//     }

//     fn peek_next(&self) -> char {
//         if self.current + 1 >= self.characters.len() {
//             '\0'
//         } else {
//             self.characters[self.current + 1]
//         }
//     }

//     fn next(&mut self) -> char {
//         self.current += 1;
//         self.characters[self.current - 1]
//     }

//     fn match_char(&mut self, expected: char) -> bool {
//         if self.is_at_end() {
//             return false;
//         }
//         if self.characters[self.current] != expected {
//             return false;
//         }

//         self.current += 1;
//         return true;
//     }

//     fn make_token(&mut self, ttype: TokenType, literal: Option<Literal>) -> Token {
//         let lexeme = self.characters[self.start..self.current]
//             .iter()
//             .cloned()
//             .collect::<String>();
//         Token::new(ttype, &lexeme, literal, self.line)
//     }
// }
