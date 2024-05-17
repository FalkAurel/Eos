use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Token{
    pub token_type: TokenType,
    pub range: (u32, u32),
    pub line: u32
}


impl Token {
    pub fn get_range(&self) -> Range<usize> {
        self.range.0 as usize..self.range.1 as usize
    }

    pub fn get_strrange(&self) -> Range<usize> {
        self.range.0 as usize + 1..self.range.1 as usize - 1
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    LeftParent,
    RightParent,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    Text,
    Integer,
    Float,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Error,
    EndOfFile
}
