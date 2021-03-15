pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth{
            stack: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &[]
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        unimplemented!("result of evaluating '{}'", input)
    }
}
