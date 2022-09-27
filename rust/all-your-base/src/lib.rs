#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base == 1 || from_base == 0 {
        return Err(Error::InvalidInputBase);
    }
    if to_base == 1 || to_base == 0 {
        return Err(Error::InvalidOutputBase);
    }
    let dec = to_decimal(number, from_base)?;
    Ok(from_decimal(dec, to_base))
}

fn to_decimal(number: &[u32], from_base: u32) -> Result<u32, Error> {
    // handle the special case of an empty slice
    if number.is_empty() {
        return Ok(0);
    }

    number
        .iter()
        .map(|&digit| {
            if digit >= from_base {
                Err(Error::InvalidDigit(digit))
            } else {
                Ok(digit)
            }
        })
        .rev()
        .enumerate()
        .map(|(i, digit)| {
            if let Ok(d) = digit {
                Ok(d * (from_base.pow(i as u32)))
            } else {
                digit
            }
        })
        .sum::<Result<u32, Error>>()
}

fn from_decimal(number: u32, target_base: u32) -> Vec<u32> {
    // handle the special case of 0
    if number == 0 {
        return vec![0];
    }

    let mut n = number;

    let mut output = Vec::new();

    while n >= 1 {
        let rem = n % target_base;
        n /= target_base;

        output.insert(0, rem);
    }

    output
}
