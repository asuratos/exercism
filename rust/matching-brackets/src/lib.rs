pub struct BracketStack {
    brackets: Vec<char>,
}

impl BracketStack {
    fn new() -> Self {
        BracketStack {
            brackets: Vec::new(),
        }
    }

    fn push(&mut self, elem: char) -> Result<(), ()> {
        match elem {
            '(' | '{' | '[' => {
                //on an open bracket, add to stack
                self.brackets.push(elem);
                return Ok(());
            }
            ')' | '}' | ']' => {
                // on a close bracket, check if it pairs with the last element
                // if it does, pop that element and move to the next
                self.check_close(elem)?;
                self.brackets.pop();
                return Ok(());
            }
            _ => return Err(()),
        };
    }

    fn check_close(&self, end: char) -> Result<(), ()> {
        if let Some(i) = self.last() {
            // if a previous element exists, make sure it matches with the 
            // incoming bracket
            match i {
                '(' => {
                    if end == ')' {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                '{' => {
                    if end == '}' {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                '[' => {
                    if end == ']' {
                        Ok(())
                    } else {
                        Err(())
                    }
                }
                _ => Err(()),
            }
        } else {
            return Err(());
        }
    }

    fn last(&self) -> Option<&char> {
        self.brackets.last()
    }

    fn is_done(&self) -> bool {
        self.brackets.is_empty()
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = BracketStack::new();

    // try to load the brackets into the stack.
    // if any operation fails, return false
    if string
        .chars()
        .filter(|&c| "[{()}]".contains(c))
        .map(|c| stack.push(c))
        .any(|res| res.is_err())
    {
        return false;
    } else {
        // if everything was loaded into the stack, but it's not empty,
        // some brackets were left open
        return stack.is_done();
    }
}
