#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for i in inputs {
        match i {
            CalculatorInput::Value(n) => stack.push(*n),
            CalculatorInput::Add => stack = add(stack)?,
            CalculatorInput::Subtract => stack = subtract(stack)?,
            CalculatorInput::Multiply => stack = multiply(stack)?,
            CalculatorInput::Divide => stack = divide(stack)?,
        }
    }

    if stack.len() == 1 {
        Some(stack[0])
    } else {
        None
    }
}

fn add(mut stack: Vec<i32>) -> Option<Vec<i32>> {
    let n1 = stack.pop()?;
    let n2 = stack.pop()?;

    stack.push(n1 + n2);

    Some(stack)
}

fn subtract(mut stack: Vec<i32>) -> Option<Vec<i32>> {
    let n1 = stack.pop()?;
    let n2 = stack.pop()?;

    stack.push(n2 - n1);

    Some(stack)
}

fn divide(mut stack: Vec<i32>) -> Option<Vec<i32>> {
    let n1 = stack.pop()?;
    let n2 = stack.pop()?;

    stack.push(n2 / n1);

    Some(stack)
}

fn multiply(mut stack: Vec<i32>) -> Option<Vec<i32>> {
    let n1 = stack.pop()?;
    let n2 = stack.pop()?;

    stack.push(n1 * n2);

    Some(stack)
}
