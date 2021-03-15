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
        Forth { stack: vec![] }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        input
            .split(' ')
            .map(|c| match c.parse::<i32>() {
                Ok(num) => Ok(self.stack.push(num)),
                Err(_) => match c.to_lowercase().as_str() {
                    "+" | "-" | "/" | "*" => self.arithmetic(c),
                    "dup" => self.dup(),
                    "drop" => self.drop(),
                    "swap" => self.swap(),
                    "over" => self.over(),
                    _ => Ok(println!("{}", c)),
                },
            })
            .collect()
    }

    fn arithmetic(&mut self, op: &str) -> ForthResult {
        let x1 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;

        match op {
            "+" => Ok(self.stack.push(x1 + x2)),
            "-" => Ok(self.stack.push(x2 - x1)),
            "*" => Ok(self.stack.push(x1 * x2)),
            "/" => {
                if x1 == 0 {
                    return Err(Error::DivisionByZero);
                };
                Ok(self.stack.push(x2 / x1))
            }

            _ => Ok(println!("{}", op)),
        }
    }

    fn dup(&mut self) -> ForthResult {
        let end = self
            .stack
            .last()
            .cloned()
            .ok_or_else(|| Error::StackUnderflow)?;

        Ok(self.stack.push(end))
    }

    fn drop(&mut self) -> ForthResult {
        self.stack.pop().ok_or_else(||Error::StackUnderflow)?;

        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let x1 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
         
        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let x1 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
        
        self.stack.push(x2);
        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }
}
