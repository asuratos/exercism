use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &'static [&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();

    let full_input = input.join("");
    let input_len = full_input.len();
    let slice_len = input_len / worker_count;
    
    let handles: Vec<_> = (0..worker_count).map(|i| {
        thread::spawn(move || {
            let start = i * slice_len;
            let end = if i < worker_count - 1 {
                (i + 1) * slice_len
            } else {
                input.len()
            };
            let local_input = &full_input[start..end];
            let mut local_hashmap = HashMap::new();
            
                for c in local_input.chars() {
                    *local_hashmap.entry(c).or_insert(0) += 1;
                }
            
            local_hashmap
        })
    }).collect();
    // for thread_no in 0..worker_count {
    //     let start = thread_no * slice_len;
    //     let full_input_clone = full_input.clone();

    //     handles.push(thread::spawn(move || {
    //         let mut inthread_map = HashMap::new();
    //         full_input_clone[start..start + slice_len]
    //             .chars()
    //             .for_each(|c| *inthread_map.entry(c).or_insert(0) += 1);
    //         inthread_map
    //     }))
    // }

    for handle in handles {
        let local_hashmap = handle.join().unwrap();
        local_hashmap.iter().for_each(|(&c, count)| {
            *freq_map.entry(c).or_insert(0) += count;
        });
    }
    return freq_map;
}
