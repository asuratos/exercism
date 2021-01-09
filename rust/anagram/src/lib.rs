use std::collections::HashSet;

fn is_anagram(word: &str, candidate: &str) -> bool {
    if word.to_lowercase() == candidate.to_lowercase() {
        return false;
    }

    normalize(word) == normalize(candidate)
}

fn normalize(s: &str) -> Vec<char> {
    let mut chars = s
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();

    chars.sort();
    chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::<&'a str>::new();

    for candidate in possible_anagrams {
        if is_anagram(word, candidate) {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
