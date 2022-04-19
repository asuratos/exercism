use std::collections::HashMap;

pub type Value = i32;
// pub type Result<(), Error> = Result<(), Error>;

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
        self.words.get(&key).cloned().ok_or(Error::UnknownWord)
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    /// Takes the string and t
    fn extract_word_defs<'a>(&mut self, input: &'a str) -> Result<Vec<&'a str>, Error> {
        let mut defs = vec![];
        let mut for_eval = vec![];

        let mut reading = 0;

        // Read each element and place words into either vectors for
        // new definitions or into the string for evaluation
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
                    } else {
                        for_eval.push(elem);
                    }
                }
            };
        }

        if reading != 0 {
            return Err(Error::InvalidWord);
        };

        println!("{:?}", defs);

        for mut def in defs {
            let new_word = def.first().ok_or(Error::InvalidWord)?;

            self.words.insert(
                new_word.to_lowercase(),
                def.split_off(1)
                    .iter()
                    .map(|c| c.to_lowercase())
                    .map(|c| {
                        if self.words.contains_key(&c) {
                            self.words.get(&c).unwrap().clone()
                        } else {
                            vec![c]
                        }
                    })
                    .flatten()
                    .collect(),
            );

            if def.first().unwrap().parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }
        }

        println!("{:?}", self.words);
        println!("for_eval after parse: {:?}", for_eval);

        Ok(for_eval)
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        // start by parsing out the parts between : and ;
        // and registering into the dictionary
        println!("{:?}", self.words);
        let for_exec: Vec<&str>;
        {
            for_exec = self.extract_word_defs(input)?;
        }

        // replace all the instances of user defined words until only default
        // words are left

        // let for_exec = input.split(' ').collect::<Vec<&str>>();

        // TODO: function that takes input &str -> Vec<&str>>
        //      Removes new word definitions and adds them to the dictionary
        //      non definitions get replaced with default ones
        //

        // for_exec should be a Vec<&str>> of the command stack, fully parsed
        // and valid (only has built in words)

        // println!("for_exec: {:?}", for_exec);

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

    fn eval_word(&mut self, word: &str) -> Result<(), Error> {
        // println!("{}", word);
        let words = self.words(word.to_string())?;

        // .get(word)
        // .ok_or(Error::InvalidWord)?
        // .ok_or_else(|| Error::UnknownWord)
        let cmd_vec = words
            .iter()
            .map(|elem| elem.as_str())
            .collect::<Vec<&str>>();

        // println!("{:?}", cmd_vec);
        self.exec(cmd_vec)
    }

    fn exec(&mut self, for_exec: Vec<&str>) -> Result<(), Error> {
        for_exec
            .iter()
            .map(|c| match c.parse::<i32>() {
                Ok(num) => Ok(self.stack.push(num)),
                Err(_) => match c.to_lowercase().as_str() {
                    word if self.words.contains_key(word) => self.eval_word(word),
                    "+" | "-" | "/" | "*" => self.arithmetic(c),
                    "dup" => self.dup(),
                    "drop" => self.drop(),
                    "swap" => self.swap(),
                    "over" => self.over(),
                    _ => return Err(Error::UnknownWord),
                },
            })
            .collect()
    }

    fn arithmetic(&mut self, op: &str) -> Result<(), Error> {
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

    fn dup(&mut self) -> Result<(), Error> {
        let end = self
            .stack
            .last()
            .cloned()
            .ok_or_else(|| Error::StackUnderflow)?;

        Ok(self.stack.push(end))
    }

    fn drop(&mut self) -> Result<(), Error> {
        self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;

        Ok(())
    }

    fn swap(&mut self) -> Result<(), Error> {
        let x1 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;

        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }

    fn over(&mut self) -> Result<(), Error> {
        let x1 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or_else(|| Error::StackUnderflow)?;

        self.stack.push(x2);
        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }
}
