struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words = s.split_whitespace().collect::<Vec<&str>>();
        match words.last() {
            None => -1,
            Some(word) => word.len() as i32,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
