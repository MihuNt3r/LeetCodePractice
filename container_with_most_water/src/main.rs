use std::cmp::min;

fn main() {
    let heights = vec![1,8,6,2,5,4,8,3,7];

    let amount_of_water = Solution::max_area(heights);

    println!("Hello, world! {}", amount_of_water);
}

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_amount_of_water = 0;

        // while left < right {
        //     let length = right - left;
        //     let min_height = min(height[left], height[right]);
        //     let square = length as i32 * min_height;
        //
        //     if square > max_amount_of_water {
        //         max_amount_of_water = square;
        //     }
        // }

        for i in 0..height.len() {
            for j in i + 1..height.len() {
                println!("{} {}", i, j);
                let length = j - i;
                let min_height = min(height[j], height[i]);

                let square = length as i32 * min_height;

                if square > max_amount_of_water {
                    max_amount_of_water = square;
                }
            }
        }

        max_amount_of_water
    }
}