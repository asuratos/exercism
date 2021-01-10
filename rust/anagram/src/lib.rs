use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn is_anagram(word: &str, candidate: &str) -> bool {
    if word.to_lowercase() == candidate.to_lowercase() {
        return false;
    }

    normalize(word) == normalize(candidate)
}

fn normalize(s: &str) -> Vec<String> {
    let mut chars = s
        .graphemes(true)
        .map(str::to_lowercase)
        .collect::<Vec<String>>();

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
