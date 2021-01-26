use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_tree = BTreeMap::new();
    h.iter().for_each(|(&s, chars)| {
        chars.iter().map(|c| c.to_ascii_lowercase()).for_each(|c| {
            new_tree.insert(c, s);
        });
    });
    new_tree
}
