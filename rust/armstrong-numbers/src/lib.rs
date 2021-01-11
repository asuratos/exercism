pub fn is_armstrong_number(num: u32) -> bool {
    let digits = (num as f32).log10() as u32 + 1;
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(digits))
        .sum::<u32>()
        == num
}
