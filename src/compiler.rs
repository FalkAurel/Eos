use std::vec::IntoIter;

use super::chunk::Chunk;
use super::common::{compile_error, SharedData};
use super::opcode::OpCode;
use super::precedence::Precedence::{self, *};
use super::token::{Token, TokenType};
use super::value::Value;

use writing_to_chunk::*;

type ParseFn = fn(&mut Compiler);

struct ParseRule
{
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence
}

const fn rule(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> ParseRule{
    ParseRule{prefix, infix, precedence}
}

const RULES: [ParseRule; 41] = [
    rule(Some(Compiler::grouping), None, NONE), // TOKEN_LEFT_PAREN
    rule(None, None, NONE),                     // TOKEN_RIGHT_PAREN
    rule(None, None, NONE),                     // TOKEN_LEFT_BRACE
    rule(None, None, NONE),                     // TOKEN_RIGHT_BRACE
    rule(None, None, NONE),                     // TOKEN_COMMA
    rule(None, None, NONE),                     // TOKEN_DOT
    rule(Some(Compiler::unary), Some(Compiler::binary), TERM), // TOKEN_MINUS
    rule(None, Some(Compiler::binary), TERM),   // TOKEN_PLUS
    rule(None, None, NONE),                     // TOKEN_SEMICOLON
    rule(None, Some(Compiler::binary), FACTOR), // TOKEN_SLASH
    rule(None, Some(Compiler::binary), FACTOR), // TOKEN_STAR
    rule(Some(Compiler::unary), None, NONE),                     // TOKEN_BANG
    rule(None, Some(Compiler::binary), EQUALITY),                     // TOKEN_BANG_EQUAL
    rule(None, None, NONE),                     // TOKEN_EQUAL
    rule(None, Some(Compiler::binary), EQUALITY),                     // TOKEN_EQUAL_EQUAL
    rule(None, Some(Compiler::binary), COMPARISON),                     // TOKEN_GREATER
    rule(None, Some(Compiler::binary), COMPARISON),                     // TOKEN_GREATER_EQUAL
    rule(None, Some(Compiler::binary), COMPARISON),                     // TOKEN_LESS
    rule(None, Some(Compiler::binary), COMPARISON),                     // TOKEN_LESS_EQUAL
    rule(None, None, NONE),                     // TOKEN_IDENTIFIER
    rule(Some(Compiler::string), None, NONE),                     // TOKEN_STRING
    rule(Some(Compiler::add_number), None, NONE),   // TOKEN_INTEGER
    rule(Some(Compiler::add_number), None, NONE),   //TOKEN_FLOAT
    rule(None, None, NONE),                     // TOKEN_AND
    rule(None, None, NONE),                     // TOKEN_CLASS
    rule(None, None, NONE),                     // TOKEN_ELSE
    rule(Some(Compiler::literal), None, NONE),                     // TOKEN_FALSE
    rule(None, None, NONE),                     // TOKEN_FOR
    rule(None, None, NONE),                     // TOKEN_FUN
    rule(None, None, NONE),                     // TOKEN_IF
    rule(Some(Compiler::literal), None, NONE),                     // TOKEN_NUL
    rule(None, None, NONE),                     // TOKEN_OR
    rule(None, None, NONE),                     // TOKEN_PRINT
    rule(None, None, NONE),                     // TOKEN_RETURN
    rule(None, None, NONE),                     // TOKEN_SUPER
    rule(None, None, NONE),                     // TOKEN_THIS
    rule(Some(Compiler::literal), None, NONE),                     // TOKEN_TRUE
    rule(None, None, NONE),                     // TOKEN_VAR
    rule(None, None, NONE),                     // TOKEN_WHILE
    rule(None, None, NONE),                     // TOKEN_ERROR
    rule(None, None, NONE),                     // TOKEN_EOF
];

fn get_rule<'a>(ttype: TokenType) -> Option<&'a ParseRule>{
    RULES.get(ttype as usize)
}


pub struct Compiler{
    tokens: IntoIter<Token>,
    source: SharedData<String>,
    chunk: SharedData<Chunk>,
    previous: Option<Token>,
    current: Option<Token>,
    line: u32,
    had_error: bool
}

impl Compiler {
    pub fn new(tokens: Vec<Token>, source: SharedData<String>, chunk: SharedData<Chunk>) -> Self{
        Self { tokens: tokens.into_iter(), source, chunk, previous: None, current: None, line: 0, had_error: false}
    }

    pub fn compile(&mut self) -> Option<()>{
        self.advance();
        self.expression();
        self.consume(TokenType::EndOfFile, "Expected end of expression.");

        if self.had_error {
            return None;
        }
        Some(())
    }

    fn parse_precedence(&mut self, precedence: Precedence){
        self.advance();
        if let Some(previous_token) = &self.previous{
            if let Some(rule) = get_rule(previous_token.token_type) {
                if let Some(prefix) = rule.prefix {
                    prefix(self);
                    while let Some(token) = &self.current {
                        if let Some(rule) = get_rule(token.token_type) {
                            if rule.precedence >= precedence {
                                self.advance(); // advancing the loop

                                let rule: &ParseRule = get_rule(self.previous.unwrap().token_type).unwrap(); // Shouldnt Fail
                                let infix: ParseFn = rule.infix.unwrap(); // Shouldnt Fail; previous is guranteed by while let Some(token) = self.current

                                infix(self);
                            } else {
                                break; // this should terminate the loop as soon as we encounter something of lower precedence
                            }
                        } else {
                            break; // this should terminate the loop as soon as we encounter an error in the grammar defined by the token
                        }
                    }
                }
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::ASSIGNMENT);
    }

    fn grouping(&mut self){
        self.expression();
        self.consume(TokenType::RightParent, "Expected )");
    }

    fn string(&mut self){
        if let Some(token) = &self.previous {
            write_value(self.chunk.as_mut(), token, self.source.as_ref());
        } else {
            self.error("Expected a non-empty Token")
        }
    }

    fn literal(&mut self){
        if let Some(token) = &self.previous {
            match token.token_type {
                TokenType::False => write_value(self.chunk.as_mut(), token, self.source.as_ref()),
                TokenType::True => write_value(self.chunk.as_mut(), token, self.source.as_ref()),
                TokenType::Null => write_value(self.chunk.as_mut(), token, self.source.as_ref()),
                _ => self.error("Expected Null or a Boolean")
            }
        } else {
            self.error("Can't be a None-Token");
        }
    }

    fn add_number(&mut self){
        //Wrapper function to make write_value conform with PraseFn's signature.
        if let Some(token) = &self.previous{
            write_value(self.chunk.as_mut(), token, self.source.as_ref());
        } else {
            self.error("Expected a non-empty Token")
        }
    }

    fn unary(&mut self){
        if let Some(token) = self.previous {
            let operator: TokenType = token.token_type;

            self.parse_precedence(Precedence::UNARY); //self.expression();

            match operator {
                TokenType::Minus | TokenType::Bang => write_opcode(self.chunk.as_mut(), OpCode::Negate, token.line),
                _ => self.error(&format!("Expected Minus but got {:?}", operator))
            }
        } else {
            self.error("Expected non-empty Token");
        }
    }

    fn binary(&mut self){
        if let Some(token) = self.previous {
            let operator: TokenType = token.token_type;

            let rule: &ParseRule = get_rule(operator).expect("Check Tokentypes");

            self.parse_precedence(rule.precedence.next());

            match operator {
                TokenType::BangEqual => write_opcodes(self.chunk.as_mut(), OpCode::Equal, OpCode::Negate, token.line),
                TokenType::EqualEqual => write_opcode(self.chunk.as_mut(), OpCode::Equal, token.line),
                TokenType::Greater => write_opcode(self.chunk.as_mut(), OpCode::Greater, token.line),
                TokenType::GreaterEqual => write_opcodes(self.chunk.as_mut(), OpCode::Less, OpCode::Negate, token.line),
                TokenType::Less => write_opcode(self.chunk.as_mut(), OpCode::Less, token.line),
                TokenType::LessEqual => write_opcodes(self.chunk.as_mut(), OpCode::Greater, OpCode::Negate, token.line),
                TokenType::Plus => write_opcode(self.chunk.as_mut(), OpCode::Add, token.line),
                TokenType::Minus => write_opcode(self.chunk.as_mut(), OpCode::Subtract, token.line),
                TokenType::Star => write_opcode(self.chunk.as_mut(), OpCode::Multiply, token.line),
                TokenType::Slash => write_opcode(self.chunk.as_mut(), OpCode::Divide, token.line),
                _ => self.error("Encountered invalid operator for binary operation")
            }
        } else {
            self.error("Expected a non-empty Token");
        }
    }

    fn advance(&mut self){
        self.previous = self.current;

        while let Some(token) = &self.tokens.next() {
            self.line = token.line;
            self.current = Some(*token);

            match token.token_type {
                TokenType::Error => self.error("Error"),
                _ => break
            }
        }
    }

    fn consume(&mut self, ttype: TokenType, error_msg: &str){
        match &self.current {
            Some(token) if token.token_type == ttype => self.advance(),
            _ => self.error(error_msg)
        }
    }

    fn error(&mut self, msg: &str){
        if !self.had_error{
            let error: &str  = &self.source.as_ref()[self.previous.unwrap().get_range()];
            compile_error(&format!("At line {}: '{}' -> {}", self.line, error, msg));
        } else {
            self.had_error = true;
        }
    }
}

mod writing_to_chunk {
    use std::{ops::Range, str::FromStr};
    use crate::token::Token;

    use super::{Chunk, OpCode, TokenType, Value};

    pub fn write_opcode(chunk: &mut Chunk, opcode: OpCode, line: u32){
        chunk.add_opcode(opcode, line);
    }

    pub fn write_opcodes(chunk: &mut Chunk, opcode1: OpCode, opcode2: OpCode, line: u32){
        chunk.add_opcode(opcode1, line);
        chunk.add_opcode(opcode2, line);
    }

    pub fn write_value(chunk: &mut Chunk, token: &Token, source: &str){
        if let Some(value) = match token.token_type {
            TokenType::Integer => Some(Value::Integer(extract_value(source, token.get_range()).unwrap())),
            TokenType::Float => Some(Value::Float(extract_value(source, token.get_range()).unwrap())),
            TokenType::True => Some(Value::Boolean(true)),
            TokenType::False => Some(Value::Boolean(false)),
            TokenType::Null => Some(Value::Null),
            TokenType::Text => Some(Value::Obj(Box::new(source[token.get_strrange()].to_string()))),
            _ => None // prevents the chunk.add_value() from being executed
        }{
            chunk.add_value(value, token.line);
        }
    }

    fn extract_value<T: FromStr>(source: &str, range: Range<usize>) -> Option<T>{
        source.get(range)?.parse::<T>().ok()
    }
}
