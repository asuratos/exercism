pub fn abbreviate(phrase: &str) -> String {
    phrase.split_whitespace()
        .flat_map(|word| word.chars().nth(0).unwrap().to_uppercase())
        .collect::<String>()
}
