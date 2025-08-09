use std::collections::HashMap;

fn main() {
    let string = "a".to_string();
    let string_t = "aa".to_string();

    let solution = Solution::min_window(string, string_t);

    println!("{}", solution);
}

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut minimum_window_substring = "";

        if s.is_empty() {
            return "".to_string();
        }


        let mut target_counts = HashMap::new();
        for c in t.chars() {
            *target_counts.entry(c).or_insert(0) += 1;
        }

        let original_string_length = s.len();
        let mut left = 0;
        let mut right = 1;

        while right <= original_string_length {
            println!("left {}, right {}", left, right);
            let current_slice = &s[left..right];

            let contains_all_characters = t
                .chars()
                .all(|char| current_slice.contains(char));

            let contains_all_characters = has_all_chars(current_slice, &target_counts);
            println!("slice: {} {}", current_slice, contains_all_characters);

            let current_slice_shorter_that_minimum_window = minimum_window_substring.is_empty() ||
                current_slice.len() < minimum_window_substring.len();

            match (contains_all_characters, current_slice_shorter_that_minimum_window) {
                (true, true) => {
                    minimum_window_substring = current_slice;
                    left += 1;
                },
                (true, false) => {
                    left += 1;
                },
                _ => {
                    right += 1;
                }
            }
            //
            // if contains_all_characters {
            //     left += 1;
            // } else {
            //     right += 1;
            // }
        }

        minimum_window_substring.to_string()
    }
}

fn has_all_chars(window: &str, target_counts: &HashMap<char, usize>) -> bool {
    let mut window_counts = HashMap::new();
    for c in window.chars() {
        *window_counts.entry(c).or_insert(0) += 1;
    }
    for (ch, &count) in target_counts {
        if window_counts.get(ch).unwrap_or(&0) < &count {
            return false;
        }
    }
    true
}
