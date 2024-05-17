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

use common::ENABLE_TESTING;
use test::run_tests;

fn main(){
    if ENABLE_TESTING {
        run_tests();
    }
}
