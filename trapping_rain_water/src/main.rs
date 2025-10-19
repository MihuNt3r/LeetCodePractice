struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = height.len()-1;
        let mut vol: i32 = 0;
        let mut max_l: i32 = 0;
        let mut max_r: i32 = 0;

        while l < r {
            if (height[l] < height[r]){
                if (max_l > height[l]){
                    vol += max_l - height[l];
                }
                else {
                    max_l = height[l];
                }
                l += 1;
            }
            else {
                if (max_r > height[r]){
                    vol += max_r - height[r];
                }
                else {
                    max_r = height[r];
                }
                r -= 1;
            }
        }

        vol
    }
}

fn main() {
    println!("Hello, world!");
}
