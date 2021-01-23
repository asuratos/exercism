use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    let mut full_input = input.join("");
    full_input.make_ascii_lowercase();

    let input_len = full_input.len();
    let slice_len = input_len / worker_count;

    let handles: Vec<_> = (0..worker_count)
        .map(|i| {
            let full_input_clone = full_input.clone();
            thread::spawn(move || {
                let start = i * slice_len;
                let end = if i < worker_count - 1 {
                    (i + 1) * slice_len
                } else {
                    full_input_clone.len()
                };
                let local_input = &full_input_clone[start..end];
                let mut local_hashmap = HashMap::new();

                local_input
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .for_each(|c| {
                        *local_hashmap.entry(c).or_insert(0) += 1;
                    });

                local_hashmap
            })
        })
        .collect();

    for handle in handles {
        let local_hashmap = handle.join().unwrap();
        local_hashmap.iter().for_each(|(&c, count)| {
            *freq_map.entry(c).or_insert(0) += count;
        });
    }
    freq_map
}
