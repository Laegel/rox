

#[derive(Debug, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        use Token::*;
        match self {
            Number(value)  => value.to_string(),
            EqualEqual => "==".to_string(),
            Equal => "=".to_string(),
            Star => "*".to_string(),
            Minus => "-".to_string(),
            Plus => "+".to_string(),

            _ => "Any".to_string()
        }
    }
}



#[derive(Debug)]
pub struct Item {
    pub token: Token,
    pub lexeme: String,
    pub line: usize,
}

impl Item {

    pub fn new(lexeme: String, token: Token, line: usize ) -> Item {
        Item {
            lexeme,
            token,
            line
        }
    }

    pub fn to_string(self) -> String {
        format!("{:?} {} {}", self.token, self.lexeme, self.line)
    }
}