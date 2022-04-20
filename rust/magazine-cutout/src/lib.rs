use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // construct hashmap of available words
    let mut mag_words: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        *mag_words.entry(word).or_insert(0) += 1;
    }

    // go through desired note and "take" words from the hashmap
    for note_word in note {
        if let Some(count) = mag_words.get_mut(note_word) {
            // if there are not enough words on the mag, return false
            if *count < 1 {
                return false;
            }
            *count -= 1;
        } else {
            //if the word is not found, return false
            return false;
        }
    }

    true
}
