use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"GTAC".contains(nucleotide) {
        return Err(nucleotide);
    };

    dna.chars().try_fold(0, |acc, c| match c {
        x if x == nucleotide => Ok(acc + 1),
        x if "GTAC".contains(x) => Ok(acc),
        x => Err(x),
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut init_map = HashMap::new();
    init_map.insert('A', 0);
    init_map.insert('G', 0);
    init_map.insert('T', 0);
    init_map.insert('C', 0);

    dna.chars()
        .try_fold(init_map, |mut acc, c| match c {
            x if "GTAC".contains(x) => {
                let count = acc.entry(x).or_insert(0);
                *count += 1;
                Ok(acc)
                },
            x => Err(x),
        })
}
