fn luhn_double(digit: char) -> Result<u32, String> {
    char::to_digit(digit, 10)
        .ok_or("Not a digit!".to_string())
        .map(|i| match i * 2 {
            i if i > 9 => i - 9,
            i => i,
        })
}

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cleaned: String = code.chars().filter(|&c| !char::is_whitespace(c)).collect();

    if cleaned.len() == 1 {
        return false;
    }

    cleaned
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 1 {
                luhn_double(c)
            } else {
                char::to_digit(c, 10).ok_or("Not a digit!".to_string())
            }
        })
        .collect::<Result<Vec<_>, _>>()
        .map_or_else(|_| false, |o| o.iter().sum::<u32>() % 10 == 0)
}
