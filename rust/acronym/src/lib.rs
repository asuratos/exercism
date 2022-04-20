pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split_ascii_whitespace()
        .flat_map(|word| word.split('-'))
        .map(|word| reduce_to_acronym(word))
        .collect::<String>()
}

fn reduce_to_acronym(word: &str) -> String {
    let mut acronym = String::new();


    // if left with empty string, return empty
    if word.is_empty() {
        return "".to_string();
    };

    // if word is all caps, return only first
    if word.to_uppercase() == *word.to_string() {
        return word.chars().next().unwrap().to_string();
    }

    // catch emphasis
    let mut wordstr = String::from(word);
    if wordstr.starts_with('_') && wordstr.ends_with('_') {
        wordstr = wordstr.strip_prefix('_').unwrap().to_string();
    }

    let mut chars = wordstr.chars();
    acronym.push(chars.next().unwrap());

    for c in chars {
        if c.is_uppercase() {
            acronym.push(c)
        }
    }

    acronym.to_ascii_uppercase()
}
