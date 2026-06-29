use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let counter = |input: &[&str]| {
        let mut map: HashMap<char, usize> = HashMap::new();
        for line in input {
            for c in line
                .chars()
                .filter(|c| c.is_alphabetic())
                .map(|c| c.to_ascii_lowercase())
            {
                *map.entry(c).or_default() += 1;
            }
        }
        map
    };
    match input.len() {
        0 => HashMap::new(),
        n if n < 500 => counter(input),
        _ => thread::scope(|s| {
            let mut handles = Vec::with_capacity(worker_count);
            for lines in input.chunks(input.len() / worker_count + 1) {
                handles.push(s.spawn(|| counter(lines)));
            }
            let mut map = HashMap::new();
            for h in handles {
                h.join().unwrap().iter().for_each(|(k, v)| {
                    *map.entry(*k).or_default() += v;
                });

            }
            map

        }),
    }
}

#[test]
fn one_text_with_multiple_letters() {
    let mut hm = HashMap::new();
    hm.insert('b', 2);
    hm.insert('c', 3);
    hm.insert('d', 1);
    let res = frequency(&["bbcccd"], 4);
    println!("result: {:#?} ", res);
    assert_eq!(res, hm);
}
