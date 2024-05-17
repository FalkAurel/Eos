use std::ops::Range;

use super::token::{Token, TokenType::{self, *}};

const TAB_BYTE: u8 = 0x9;

const fn make_token(lexer: &Lexer, token_type: TokenType) -> Token {
    Token { token_type, range: (lexer.start as u32, lexer.current as u32), line: lexer.line }
}

pub struct Lexer<'a>{
    source: &'a [u8],
    current: usize,
    start: usize,
    line: u32
}


impl <'a> Lexer <'a>{
    pub fn new(source: &'a str) -> Self {
        Self {source: source.as_bytes(), current: 0, start: 0, line: 1 }
    }

    pub fn next_token(&mut self) -> Token{
        self.skip_whitespaces();
        self.skip_comment();
        self.skip_whitespaces();
        self.tokenize()
    }

    pub fn lexing(&mut self) -> Vec<Token>{
        let mut output: Vec<Token> = Vec::new();
        loop {
            let token: Token = self.next_token();
            output.push(token);
            if token.token_type == EndOfFile || token.token_type == Error {
                return output;
            }
        }
    }

    fn tokenize(&mut self) -> Token {
        self.start = self.current;

        if let Some(next_char) = self.advance() {
            if next_char.is_ascii_alphabetic() {
                match next_char {
                    b'v' if self.match_pattern(b"var") => return make_token(self, Var),
                    b'a' if self.match_pattern(b"and") => return make_token(self, And),
                    b'i' if self.match_pattern(b"if") => return make_token(self, If),
                    b'e' if self.match_pattern(b"else") => return make_token(self, Else),
                    b'N' if self.match_pattern(b"Null") => return make_token(self, Null),
                    b'o' if self.match_pattern(b"or") => return make_token(self, Or),
                    b'p' if self.match_pattern(b"print") => return make_token(self, Print),
                    b'r' if self.match_pattern(b"return") => return make_token(self, Return),
                    b'w' if self.match_pattern(b"while") => return make_token(self, While),
                    b'c' if self.match_pattern(b"class") => return make_token(self, Class),
                    b's' if self.match_pattern(b"super") => return make_token(self, Super),
                    b'f' if self.match_pattern(b"false") => return make_token(self, False),
                    b'f' if self.match_pattern(b"for") => return make_token(self, For),
                    b'f' if self.match_pattern(b"fun") => return make_token(self, Fun),
                    b't' if self.match_pattern(b"this") => return make_token(self, This),
                    b't' if self.match_pattern(b"true") => return make_token(self, True),
                    _ => return self.parse_identifier()
                }
            } else if next_char.is_ascii_digit() {
                return self.parse_number();
            } else {
                match next_char {
                    b'(' => return make_token(self, LeftParent),
                    b')' => return make_token(self, RightParent),
                    b'{' => return make_token(self, LeftBrace),
                    b'}' => return make_token(self, RightBrace),
                    b'+' => return make_token(self, Plus),
                    b'-' => return make_token(self, Minus),
                    b'*' => return make_token(self, Star),
                    b'/' => return make_token(self, Slash),
                    b',' => return make_token(self, Comma),
                    b'.' => return make_token(self, Dot),
                    b';' => return make_token(self, Semicolon),
                    b'"' => return self.parse_string(),
                    b'!' if self.match_pattern(b"!=") => return make_token(self, BangEqual),
                    b'!' => {println!("Bang"); return make_token(self, Bang)},
                    b'=' if self.match_pattern(b"==") => return make_token(self, EqualEqual),
                    b'=' => return make_token(self, Equal),
                    b'>' if self.match_pattern(b">=") => return make_token(self, GreaterEqual),
                    b'>' => return make_token(self, Greater),
                    b'<' if self.match_pattern(b"<=") => return make_token(self, LessEqual),
                    b'<' => return make_token(self, Less),
                    _ => return make_token(&self, Error),
                }
            }
        }
        make_token(self, EndOfFile)
    }

    fn skip_whitespaces(&mut self){
        while let Some(next_char) = self.peek() {
            match *next_char {
                b' ' | TAB_BYTE => {self.advance();},
                b'\n' => {
                    self.line += 1;
                    self.advance();
                },
                _ => break
            }
        }
    }

    fn skip_comment(&mut self) {
        if let Some(next_char) = self.peek() {

            if *next_char == b'#' {
                self.advance();

                while let Some(next_byte) = self.advance() {
                    if next_byte == b'\n' {
                        self.line += 1;
                        break;
                    }
                }
            }
        }
    }

    fn peek(&self) -> Option<&u8> {
        self.source.get(self.current)
    }

    fn advance(&mut self) -> Option<u8> {
        if let Some(next_char) = self.source.get(self.current) {
            self.current += 1;
            return Some(*next_char);
        }
        None
    }

    fn match_pattern<T: PartialEq<u8>>(&mut self, pattern: &[T]) -> bool {
        if self.start + pattern.len() < self.source.len() {
            let range: Range<usize> = self.start..self.start + pattern.len();

            if *pattern == self.source[range] {
                self.current += pattern.len() - 1;
                return true;
            }
        }
        false
    }

    fn parse_string(&mut self) -> Token{
        while let Some(next_char) = self.advance() {
            if next_char == b'"' {
                return make_token(self, Text);
            }
        }
        make_token(self, Error)
    }

    fn parse_identifier(&mut self) -> Token{
        while let Some(next_char) = self.peek() {
            match *next_char {
                temp if temp.is_ascii_alphanumeric() || temp == b'_' => {self.current += 1},
                _ => return make_token(self, Identifier)
            }
        }
        make_token(self, Error)
    }

    fn parse_number(&mut self) -> Token{
        let mut is_float: bool = false;

        while let Some(next_char) = self.peek() {
            match next_char {
                b'0'..=b'9' => {self.current += 1;},
                b'.' if !is_float => {is_float = true; self.current += 1},
                b'.' if is_float => break,
                b' ' | b'\n' => {if is_float {return make_token(self, Float);} else {return make_token(self, Integer)}},
                _ => break
            }
        }
        make_token(self, Error)
    }
}
