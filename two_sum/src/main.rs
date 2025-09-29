use std::collections::HashMap;

struct Solution;

impl Solution {
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut map: HashMap<i32, i32> = HashMap::new();
    //     let mut result = vec![];
    //
    //     nums.iter().enumerate().for_each(|(num, index)| {
    //         let remaining = target - num as i32;
    //         map.insert(remaining, *index);
    //     });
    //
    //     nums.iter().enumerate().for_each(|(num, index)| {
    //         let num = num as i32;
    //         if let Some(remainder_index) = map.get(&num) {
    //             result = vec![*index, *remainder_index];
    //         }
    //     });
    //
    //     result
    // }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                result = vec![j, i as i32];
            }
            map.insert(num, i as i32);
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}
