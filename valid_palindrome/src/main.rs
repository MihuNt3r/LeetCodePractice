fn main() {
    let result = Solution::is_palindrome("A man, a plan, a canal: Panama".to_string());

    println!("Hello, world! {}", result);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let alphanumeric_chars: Vec<char> = s
            .chars()
            .filter_map(|char| {
                if char.is_alphanumeric() {
                    Some(char.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .collect();

        // println!("{}", alphanumeric_string);
        // let chars_count = alphanumeric_string.len();
        // let mut last_char_index = chars_count - 1;
        //
        // for i in 0..chars_count {
        //     if alphanumeric_string[i] != alphanumeric_string[last_char_index] {
        //         return false;
        //     }
        //     last_char_index -= 1;
        // }

        let mut left = 0;
        let mut right = alphanumeric_chars.len().saturating_sub(1); // avoids panic if empty

        while left < right {
            if alphanumeric_chars[left] != alphanumeric_chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}
