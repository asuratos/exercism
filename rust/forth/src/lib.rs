use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub const BUILTIN: [&str; 4] = ["dup", "drop", "swap", "over"];

pub struct Forth {
    stack: Vec<i32>,
    words: HashMap<String, Vec<String>>,
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
        Forth {
            stack: vec![],
            words: HashMap::new(),
        }
    }

    pub fn words(&self, key: String) -> Result<Vec<String>, Error> {
        self.words.get(&key).cloned().ok_or(Error::InvalidWord)
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn extract_word_defs(&mut self, input: &str) -> ForthResult {
        let mut defs = vec![];

        let mut reading = 0;

        for elem in input.split(' ') {
            match elem {
                ":" => {
                    reading += 1;
                    defs.push(vec![]);
                }
                ";" => {
                    reading -= 1;
                }
                _ => {
                    if (reading > 1) | (reading < 0) {
                        return Err(Error::InvalidWord);
                    }
                    if reading == 1 {
                        defs.last_mut().unwrap().push(String::from(elem));
                    }
                }
            };
        }

        println!("{:?}", defs);

        for mut def in defs {
            let new_word = def.first().ok_or(Error::InvalidWord)?;

            self.words.insert(new_word.to_string(), def.split_off(1));

            if def.first().unwrap().parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }


        }

        println!("{:?}", self.words);

        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        // start by parsing out the parts between : and ;
        // and registering into the dictionary
        self.extract_word_defs(input)?;

        // replace all the instances of user defined words until only default
        // words are left

        let for_exec = input.split(' ').collect::<Vec<&str>>();

        // for_exec should be a Vec<&str>> of the command stack, fully parsed
        // and valid (only has built in words)

        self.exec(for_exec)
        // for_eval
        //     .iter()
        //     .map(|c| match c.parse::<i32>() {
        //         Ok(num) => Ok(self.stack.push(num)),
        //         Err(_) => match c.to_lowercase().as_str() {
        //             "+" | "-" | "/" | "*" => self.arithmetic(c),
        //             "dup" => self.dup(),
        //             "drop" => self.drop(),
        //             "swap" => self.swap(),
        //             "over" => self.over(),
        //             wrd => self.eval_word(wrd),
        //         },
        //     })
        //     .collect()
    }

    fn eval_word(&mut self, word: &str) -> ForthResult {
        // println!("{}", word);
        let cmd_vec = self
                .words(word.to_string())?
                // .get(word)
                // .ok_or(Error::InvalidWord)?
                // .ok_or_else(|| Error::UnknownWord)
                .iter()
                .map(|elem| elem.as_str())
                .collect::<Vec<&str>>();


        // println!("{:?}", cmd_vec);
        self.exec(cmd_vec);
        Ok(())
    }

    fn exec(&mut self, for_exec: Vec<&str>) -> ForthResult {
        for_exec
            .iter()
            .map(|c| match c.parse::<i32>() {
                Ok(num) => Ok(self.stack.push(num)),
                Err(_) => match c.to_lowercase().as_str() {
                    "+" | "-" | "/" | "*" => self.arithmetic(c),
                    "dup" => self.dup(),
                    "drop" => self.drop(),
                    "swap" => self.swap(),
                    "over" => self.over(),
                    word => self.eval_word(word),
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
        self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;

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
