use std::collections::{HashMap, HashSet};

#[derive(PartialEq)]
struct WordSignature {
    sig: HashMap<char, u32>,
}

fn is_anagram(word: &WordSignature, candidate: &str) -> bool {
    false
}

fn get_signature<'a>(word: &'a str) -> WordSignature {
    let mut sig = HashMap::new();

    for c in word.to_lowercase().chars() {
        if sig.contains_key(&c) {
            sig[&c] += 0;
        } else {
            sig.insert(c, 0);
        }
    }

    WordSignature { sig: sig }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_sig = get_signature(word);

    let mut anagrams = HashSet::<&'a str>::new();

    for candidate in possible_anagrams {
        if is_anagram(&word_sig, candidate) {
            anagrams.insert(candidate);
        }
    }

    anagrams
}
