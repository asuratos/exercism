pub type Value = i32;

pub type ForthResult = Result<(), Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}
pub struct Forth {
    stack: Vec<Value>,
    words: Vec<Definition>,
}

#[derive(Clone)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Drop,
    Over,
    Number(Value),
    Call(usize),
}

pub struct Definition {
    name: String,
    body: Vec<Instruction>,
}

fn parse_builtin(word: &str) -> Result<Instruction, Error> {
    match word {
        "+" => Ok(Instruction::Add),
        "-" => Ok(Instruction::Sub),
        "*" => Ok(Instruction::Mul),
        "/" => Ok(Instruction::Div),
        "dup" => Ok(Instruction::Dup),
        "swap" => Ok(Instruction::Swap),
        "drop" => Ok(Instruction::Drop),
        "over" => Ok(Instruction::Over),
        _ => {
            if let Ok(num) = word.parse::<Value>() {
                Ok(Instruction::Number(num))
            } else {
                Err(Error::UnknownWord)
            }
        }
    }
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            words: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn parse_word<'a>(
        &mut self,
        word: &'a str,
        remaining_input: &mut impl Iterator<Item = &'a str>,
    ) -> ForthResult {
        if word == ":" {
            self.parse_definition(remaining_input)
        } else {
            let instr = self.parse_normal_word(word)?;
            self.eval_instruction(instr)
        }
    }

    fn parse_definition<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> ForthResult {
        if let Some(new_word) = iter.next() {
            if new_word.parse::<Value>().is_ok() {
                return Err(Error::InvalidWord);
            }

            let name = new_word.to_ascii_lowercase();
            let mut body = vec![];
            for word in iter {
                if word == ";" {
                    self.words.push(Definition { name, body });
                    return Ok(());
                } else {
                    body.push(self.parse_normal_word(word)?)
                }
            }
        }
        Err(Error::InvalidWord)
    }

    fn parse_normal_word(&mut self, word: &str) -> Result<Instruction, Error> {
        if word == ":" || word == ";" {
            Err(Error::InvalidWord)
        } else {
            let lowercase = word.to_ascii_lowercase();
            if let Some(call) = self.find_defn(&lowercase) {
                Ok(call)
            } else {
                parse_builtin(&lowercase)
            }
        }
    }

    fn eval_instruction(&mut self, instr: Instruction) -> ForthResult {
        match instr {
            Instruction::Add => self.arithmetic("+"),
            Instruction::Sub => self.arithmetic("-"),
            Instruction::Mul => self.arithmetic("*"),
            Instruction::Div => self.arithmetic("/"),
            Instruction::Dup => self.dup(),
            Instruction::Swap => self.swap(),
            Instruction::Drop => self.drop(),
            Instruction::Over => self.over(),
            Instruction::Number(n) => {
                self.stack.push(n);
                Ok(())
            }
            Instruction::Call(idx) => self.call(idx),
        }
    }

    fn call(&mut self, idx: usize) -> ForthResult {
        let def = self.words[idx].body.clone();
        for instr in def {
            self.eval_instruction(instr)?;
        }
        Ok(())
    }

    fn find_defn(&self, word: &str) -> Option<Instruction> {
        for (idx, defn) in self.words.iter().enumerate().rev() {
            if defn.name == word {
                return Some(Instruction::Call(idx));
            }
        }
        None
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let mut iter = input.split_ascii_whitespace();

        while let Some(word) = iter.next() {
            self.parse_word(word, &mut iter)?;
        }
        Ok(())
    }

    fn arithmetic(&mut self, op: &str) -> ForthResult {
        let x1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or(Error::StackUnderflow)?;

        match op {
            "+" => self.stack.push(x1 + x2),
            "-" => self.stack.push(x2 - x1),
            "*" => self.stack.push(x1 * x2),
            "/" => {
                if x1 == 0 {
                    return Err(Error::DivisionByZero);
                };

                self.stack.push(x2 / x1);
            }

            _ => (),
        };
        Ok(())
    }

    fn dup(&mut self) -> ForthResult {
        let end = self.stack.last().cloned().ok_or(Error::StackUnderflow)?;

        self.stack.push(end);
        Ok(())
    }

    fn drop(&mut self) -> ForthResult {
        self.stack.pop().ok_or(Error::StackUnderflow)?;

        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let x1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or(Error::StackUnderflow)?;

        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let x1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or(Error::StackUnderflow)?;

        self.stack.push(x2);
        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }
}
