use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let numbers_to_characters = HashMap::from(
            [
                ('2', &["a", "b", "c"] as &[&str]),
                ('3', &["d", "e", "f"]),
                ('4', &["g", "h", "i"]),
                ('5', &["j", "k", "l"]),
                ('6', &["m", "n", "o"]),
                ('7', &["p", "q", "r", "s"]),
                ('8', &["t", "u", "v"]),
                ('9', &["w", "x", "q", "z"]),
            ],
        );

        digits.chars().fold(vec![String::new()], |acc, digit| {
            if let Some(chars) = numbers_to_characters.get(&digit) {
                acc.into_iter()
                    .flat_map(|prefix| {
                        chars.iter()
                            .map(move |&c| format!("{}{}", prefix, c))
                    })
                    .collect()
            } else {
                acc
            }
        })
    }
}