struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut array = [0; 26];
        let len = s.len();

        for i in 0..len {
            let sc = s.as_bytes()[i];
            let tc = t.as_bytes()[i];

            array[(sc - b'a') as usize] += 1;
            array[(tc - b'a') as usize] -= 1;
        }

        array.iter().all(|&elem| elem == 0)
    }
}

fn main() {
    println!("Hello, world!");
}
