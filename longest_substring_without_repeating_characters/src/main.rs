use std::cmp::max;
use std::collections::HashSet;

fn main() {
    let string = String::from("abcabcbb");

    let longest_substring_length = Solution::length_of_longest_substring(string);

    println!("Length: {}", longest_substring_length);
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let length = s.len();
        let mut left = 0;
        let mut right = 1;
        let mut longest_substring_length = 0;

        while right < length {
            let slice = &s[left..=right];
            let has_no_duplicates = has_no_duplicates(slice);

            if has_no_duplicates {
                longest_substring_length = max(longest_substring_length, slice.len());
                right += 1;
            } else {
                left += 1;
            }
        }

        longest_substring_length as i32
    }
}


fn has_no_duplicates(slice: &str) -> bool {
    let mut seen = HashSet::new();
    for ch in slice.chars() {
        if !seen.insert(ch) {
            return false; // Duplicate found
        }
    }
    true // No duplicates
}