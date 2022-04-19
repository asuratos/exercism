use std::collections::HashMap;

pub type Value = i32;

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

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn stack(&self) -> &[i32] {
        &self.stack
    }

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
                    if !(0..=1).contains(&reading) {
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

        //catch cases where : and ; are not properly paired
        if reading != 0 {
            return Err(Error::InvalidWord);
        };

        for mut def in defs {
            let new_word = def.first().ok_or(Error::InvalidWord)?.clone();

            let new_def = def
                .split_off(1)
                .iter()
                .map(|c| c.to_lowercase())
                // if a word in the definition is an already defined custom
                // word, then copy that word's definition into this
                // .map(|c| {
                //     if self.words.contains_key(&c) {
                //         self.words.get(&c).unwrap().clone()
                //     } else {
                //         vec![c]
                //     }
                // })
                // .flatten()
                .collect();

            if !self.words.contains_key(&new_word) {
                self.words.insert(new_word.to_lowercase(), new_def);
            } else {
                if let Some(old_def) = self.words.get_mut(&new_word) {
                    *old_def = new_def;
                }
            }

            if def.first().unwrap().parse::<i32>().is_ok() {
                return Err(Error::InvalidWord);
            }
        }

        Ok(for_eval)
    }

    pub fn eval(&mut self, input: &str) -> Result<(), Error> {
        // start by parsing out the parts between : and ;
        // and registering into the dictionary
        let for_exec = self.extract_word_defs(input)?;

        self.exec(for_exec)
    }

    fn eval_word(&mut self, word: &str) -> Result<(), Error> {
        let words = self.words(word.to_string())?;

        let cmd_vec = words
            .iter()
            .map(|elem| elem.as_str())
            .collect::<Vec<&str>>();

        self.exec(cmd_vec)
    }

    fn exec(&mut self, for_exec: Vec<&str>) -> Result<(), Error> {
        for_exec.iter().try_for_each(|c| match c.parse::<i32>() {
            Ok(num) => {
                self.stack.push(num);
                Ok(())
            }
            Err(_) => match c.to_lowercase().as_str() {
                //prioritize user-defined words before built-in
                word if self.words.contains_key(word) => self.eval_word(word),
                "+" | "-" | "/" | "*" => self.arithmetic(c),
                "dup" => self.dup(),
                "drop" => self.drop(),
                "swap" => self.swap(),
                "over" => self.over(),
                _ => Err(Error::UnknownWord),
            },
        })
    }

    fn arithmetic(&mut self, op: &str) -> Result<(), Error> {
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

    fn dup(&mut self) -> Result<(), Error> {
        let end = self.stack.last().cloned().ok_or(Error::StackUnderflow)?;

        self.stack.push(end);
        Ok(())
    }

    fn drop(&mut self) -> Result<(), Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)?;

        Ok(())
    }

    fn swap(&mut self) -> Result<(), Error> {
        let x1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or(Error::StackUnderflow)?;

        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }

    fn over(&mut self) -> Result<(), Error> {
        let x1 = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let x2 = self.stack.pop().ok_or(Error::StackUnderflow)?;

        self.stack.push(x2);
        self.stack.push(x1);
        self.stack.push(x2);
        Ok(())
    }
}
