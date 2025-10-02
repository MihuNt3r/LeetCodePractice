use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set = nums.iter().collect::<HashSet<&i32>>();

        set.len() != nums.len()
    }
}

fn main() {
    println!("Hello, world!");
}
