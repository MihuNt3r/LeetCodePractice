use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums_set = nums.into_iter().collect::<HashSet<i32>>();

        let mut longest_consecutive = 1;

        nums_set
            .iter()
            .for_each(|num| {
                if nums_set.contains(&(num - 1)) {
                    return;
                } else {
                    let mut current_num = *num;
                    let mut current_sub = 1;

                    while nums_set.contains(&(current_num + 1)) {
                        current_num += 1;
                        current_sub += 1;
                    }

                    longest_consecutive = std::cmp::max(current_sub, longest_consecutive);
                }


            });

        longest_consecutive
    }
}

fn main() {
    println!("Hello, world!");
}
