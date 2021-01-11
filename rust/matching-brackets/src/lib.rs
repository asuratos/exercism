fn check_for_balance(start: &str, brackets: &str) -> bool {
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets_only = string.chars()
    .filter(|&c| "[{()}]".contains(c))
    .collect::<String>();

    check_for_balance(&brackets_only[..1], &brackets_only[1..])
}
