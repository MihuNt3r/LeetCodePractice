use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return vec![];
        }

        let mut answer_map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut count = [0; 26]; // frequency array
            for c in s.chars() {
                count[(c as u8 - b'a') as usize] += 1;
            }

            // build a key like "#1#0#2..." similar to Java code
            let mut key = String::new();
            for num in &count {
                key.push('#');
                key.push_str(&num.to_string());
            }

            answer_map.entry(key).or_insert(vec![]).push(s);
        }

        answer_map.into_values().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
