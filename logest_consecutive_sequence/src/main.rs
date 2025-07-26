fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut sorted_nums = nums;
        sorted_nums.sort();

        let mut max_consecutive = 1;
        let mut current_consecutive = 1;

        // for i in 1..sorted_nums.len() {
        //     if sorted_nums[i - 1] == sorted_nums[i] - 1 || sorted_nums[i - 1] == sorted_nums[i] {
        //         current_consecutive += 1;
        //
        //         if max_consecutive < current_consecutive {
        //             max_consecutive = current_consecutive;
        //         }
        //     } else {
        //         current_consecutive = 0
        //     }
        // }

        for i in 1..sorted_nums.len() {
            if sorted_nums[i] == sorted_nums[i - 1] {
                // skip duplicate
                continue;
            } else if sorted_nums[i] == sorted_nums[i - 1] + 1 {
                current_consecutive += 1;
                max_consecutive = max_consecutive.max(current_consecutive);
            } else {
                current_consecutive = 1;
            }
        }

        max_consecutive
    }
}
