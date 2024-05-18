use testing::*;

pub fn run_tests() {
    runtime_error();
    compiler_string_manipulation();
    compiler_strings_comparison();
    compiler_boolean_expression();
    compiler_boolean();
    compiler_calculating();
    compiler_error_message();
    lexer_integer_float();
    lexer_keyword_identifier();
    lexer_string_parsing();
    lexer_one_lookahed_token();
    lexer_whitespace_comment();
    vm_binary_operations();
    vm_negate();
}


mod testing {
    use std::fs::read_to_string;
    use crate::data_structures::DynType;
    use crate::token::Token;
    use crate::{lexer::Lexer, token::TokenType};
    use crate::{vm::VM, chunk::{Chunk, print_chunk}, opcode::OpCode, value::Value, compiler::Compiler, common::{DEFAULT_STACK_CAPACITY, SharedData}};


    pub fn runtime_error(){
        let code: String = read_to_string("src/tests/testing_runtime_error.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let mut chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let mut compiler: Compiler = Compiler::new(lexer.lexing(), shared_code, SharedData::new(&chunk));

        if compiler.compile().is_some(){
            chunk.add_opcode(OpCode::Print, 0);
            let mut vm: VM = VM::new(&chunk);

            let _ = vm.run();

        } else {
            assert!(false);
        }
    }

    pub fn compiler_strings_comparison(){
        let code: String = read_to_string("src/tests/testing_compiler_string_comparision.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let mut compiler: Compiler = Compiler::new(lexer.lexing(), shared_code, SharedData::new(&chunk));

        if compiler.compile().is_some(){
            let mut vm: VM = VM::new(&chunk);

            let _ = vm.run();

            assert_eq!(vm.get_stack(), &vec![Value::Boolean(true)])//&vec![Value::Obj(ObjectString::new("Hallo Welt, anscheinend hat das funktioniert!".to_string()))])
        } else {
            assert!(false);
        }
    }

    pub fn compiler_string_manipulation(){
        let code: String = read_to_string("src/tests/testing_compiler_string_manipulation.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let mut compiler: Compiler = Compiler::new(lexer.lexing(), shared_code, SharedData::new(&chunk));

        if compiler.compile().is_some(){
            let mut vm: VM = VM::new(&chunk);

            let _ = vm.run();

            assert_eq!(vm.get_stack(), &vec![Value::Object(DynType::new("String".to_string()))])
        } else {
            assert!(false);
        }
    }

    pub fn compiler_boolean_expression(){
        let code: String = read_to_string("src/tests/testing_compiler_comparison.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let tokens: Vec<Token> = lexer.lexing();
        let mut compiler: Compiler = Compiler::new(tokens.clone(), shared_code, SharedData::new(&chunk));

        if compiler.compile().is_some(){
            let mut vm: VM = VM::new(&chunk);

            let _ = vm.run();

            assert_eq!(vm.get_stack(), vec![Value::Boolean(true)])
        } else {
            assert!(false);
        }
    }

    pub fn compiler_boolean(){
        let code: String = read_to_string("src/tests/testing_literal_compiler.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let mut compiler: Compiler = Compiler::new(lexer.lexing(), shared_code, SharedData::new(&chunk));

        if compiler.compile().is_some(){
            let mut vm: VM = VM::new(&chunk);

            let _ = vm.run();

            //println!("{:?}", vm);
            assert_eq!(vm.get_stack(), vec![Value::Boolean(false)])
        } else {
            assert!(false);
        }
    }

    pub fn compiler_calculating(){
        let code: String = read_to_string("src/tests/testing_compiler.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let temp = lexer.lexing();
        let mut compiler: Compiler = Compiler::new(temp.clone(), shared_code, SharedData::new(&chunk));

        if compiler.compile().is_some(){
            let mut vm: VM = VM::new(&chunk);

            let _ = vm.run();

            assert_eq!(vm.get_stack(), vec![Value::Integer(8)])
        } else {
            assert!(false);
        }
    }

    pub fn compiler_error_message(){
        let code: String = read_to_string("src/tests/testing_compiler_error.eos").unwrap();
        let shared_code: SharedData<String> = SharedData::new(&code);

        let mut lexer: Lexer = Lexer::new(&code);
        let chunk: Chunk = Chunk::new(DEFAULT_STACK_CAPACITY);
        let mut compiler: Compiler = Compiler::new(lexer.lexing(), shared_code, SharedData::new(&chunk));

        compiler.compile();
    }

    pub fn lexer_integer_float(){
        let code: String = read_to_string("src/tests/testing_int_float.eos").unwrap();

        let mut lexer: Lexer = Lexer::new(&code);

        assert_eq!(lexer.next_token().token_type, TokenType::Integer);
        assert_eq!(lexer.next_token().token_type, TokenType::Float);
        assert_eq!(lexer.next_token().token_type, TokenType::Error);
    }

    pub fn lexer_keyword_identifier(){
        let code: String = read_to_string("src/tests/testing_keyword_identifier.eos").unwrap();

        let mut lexer: Lexer = Lexer::new(&code);

        assert_eq!(lexer.next_token().token_type, TokenType::For);
        assert_eq!(lexer.next_token().token_type, TokenType::If);
        assert_eq!(lexer.next_token().token_type, TokenType::While);
        assert_eq!(lexer.next_token().token_type, TokenType::False);
        assert_eq!(lexer.next_token().token_type, TokenType::This);
        assert_eq!(lexer.next_token().token_type, TokenType::True);
        assert_eq!(lexer.next_token().token_type, TokenType::Identifier);
    }

    pub fn lexer_string_parsing(){
        let code: String = read_to_string("src/tests/testing_string_parsing.eos").unwrap();

        let mut lexer: Lexer = Lexer::new(&code);

        assert_eq!(lexer.next_token().token_type, TokenType::Text);
        assert_eq!(lexer.next_token().token_type, TokenType::Text);
        assert_eq!(lexer.next_token().token_type, TokenType::Error);
    }


    pub fn lexer_one_lookahed_token(){
        let code: String = read_to_string("src/tests/testing_one_lookahead.eos").unwrap();

        let mut lexer: Lexer = Lexer::new(&code);

        assert_eq!(lexer.next_token().token_type, TokenType::BangEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::EqualEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::Bang);
        assert_eq!(lexer.next_token().token_type, TokenType::GreaterEqual);
        assert_eq!(lexer.next_token().token_type, TokenType::EndOfFile);
    }

    pub fn lexer_whitespace_comment(){
        let code: String = read_to_string("src/tests/testing_whitespace_comment.eos").unwrap();

        let mut lexer: Lexer = Lexer::new(&code);

        assert_eq!(lexer.next_token().token_type, TokenType::LeftParent);
        assert_eq!(lexer.next_token().token_type, TokenType::RightParent);
        assert_eq!(lexer.next_token().token_type, TokenType::EndOfFile);
    }

    pub fn vm_binary_operations(){
        let mut chunk: Chunk = Chunk::new(1);

        chunk.add_value(Value::Integer(90), 0);
        chunk.add_value(Value::Integer(90), 0);
        chunk.add_opcode(OpCode::Add, 0);
        chunk.add_value(Value::Integer(4), 0);
        chunk.add_opcode(OpCode::Multiply, 0);

        let mut vm: VM = VM::new(&chunk);
        vm.run().unwrap();

        assert_eq!(*vm.get_stack().get(0).unwrap(), Value::Integer(180 * 4));
    }

    pub fn vm_negate(){
        let mut chunk: Chunk = Chunk::new(1);

        chunk.add_value(Value::Integer(90), 0);
        chunk.add_opcode(OpCode::Negate, 0);
        chunk.add_value(Value::Boolean(false), 0);
        chunk.add_opcode(OpCode::Negate, 0);

        let mut vm: VM = VM::new(&chunk);
        vm.run().unwrap();

        assert_eq!(vm.get_stack(), [Value::Integer(-90), Value::Boolean(true)]);
    }
}
