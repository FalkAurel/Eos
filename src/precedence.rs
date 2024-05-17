use Precedence::*;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Precedence {
    NONE,
    ASSIGNMENT, // =
    OR,
    AND,
    EQUALITY, // ==
    COMPARISON, // < > <= >=
    TERM,
    FACTOR,
    UNARY,
    CALL,
    PRIMARY,
}

impl Precedence{
    pub fn next(self) -> Self{
        match self {
            NONE => ASSIGNMENT,
            ASSIGNMENT => OR,
            OR => AND,
            AND => EQUALITY,
            EQUALITY => COMPARISON,
            COMPARISON => TERM,
            TERM => FACTOR,
            FACTOR => UNARY,
            UNARY => CALL,
            CALL => PRIMARY,
            PRIMARY => PRIMARY
        }
    }
}
