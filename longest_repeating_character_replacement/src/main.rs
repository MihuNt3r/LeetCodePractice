fn main() {
    let string = String::from("AABABBA");

    let longest_substring = Solution::character_replacement(string, 1);

    println!("Longest: {}", longest_substring);
}

struct Solution;

impl Solution {
    // pub fn character_replacement(s: String, k: i32) -> i32 {
    //     let mut left = 0;
    //     let mut right = 1;
    //     let mut longest_substring = 0;
    //
    //     while right < s.len() {
    //         let slice = &s[left..=right];
    //     }
    //
    //     1
    // }
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let mut count = [0; 26];
        let mut left = 0;
        let mut max_freq = 0;
        let mut longest_substring = 0;

        for right in 0..s.len() {
            let idx = (bytes[right] - b'A') as usize;
            count[idx] += 1;
            max_freq = max_freq.max(count[idx]);

            let window_len = right - left + 1;
            if window_len - max_freq > k as usize {
                let left_idx = (bytes[left] - b'A') as usize;
                count[left_idx] -= 1;
                left += 1;
            }

            longest_substring = longest_substring.max(right - left + 1);
        }

        longest_substring as i32
    }
}
