mod chunk;
mod common;
mod compiler;
mod data_structures;
mod lexer;
mod opcode;
mod precedence;
mod test;
mod token;
mod value;
mod vm;


use common::{DEBUG_BYTECODE, ENABLE_TESTING};
use test::run_tests;
use data_structures::initialize_interned_string;

fn main(){
    initialize_interned_string();
    if ENABLE_TESTING {
        run_tests();
    }

    if DEBUG_BYTECODE {
        //print_chunk(chunk, name)
    } else {
        // execute it
    }
}
