fn main() {
    let vec = vec![-1,0,1,2,-1,-4];

    let result = Solution::three_sum(vec);

    println!("result - {:?}", result);
}

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // let nums_with_indexes: Vec<(usize, &i32)> = nums
        //     .iter()
        //     .enumerate()
        //     .collect();
        //
        // println!("{:?}", nums_with_indexes);
        //
        // let nums_length = nums_with_indexes.len();
        // let mut triplets: Vec<(i32, i32, i32)> = vec![];
        //
        // for i in 0..nums_length {
        //     for j in 0..nums_length {
        //         for k in 0..nums_length {
        //             let sum = nums[i] + nums[j] + nums[k];
        //
        //             if is_good_triplet(i as i32, j as i32, k as i32, sum) {
        //                 let mut triplet = vec![nums[i], nums[j], nums[k]];
        //                 triplet.sort();
        //                 let [i_elem, j_elem, k_elem] = triplet[..] else {
        //                     unreachable!()
        //                 };
        //                 triplets.push((i_elem, j_elem, k_elem));
        //             }
        //         }
        //     }
        // }
        //
        // triplets.dedup();
        //
        // let triplets: Vec<Vec<i32>> = triplets
        //     .into_iter()
        //     .map(|(i, j, k)| vec![i, j, k])
        //     .collect();
        //
        // triplets

        let mut result = vec![];
        let mut nums = nums;

        nums.sort(); // sort the array

        let len = nums.len();
        for i in 0..len {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate values for i
            }

            let mut left = i + 1;
            let mut right = len - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];

                if sum == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    // skip duplicates for left
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    // skip duplicates for right
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

pub fn is_good_triplet(i: i32, j: i32, k: i32, sum: i32) -> bool {
    if i != j && i != k && j != k && sum == 0 {
        true
    } else {
        false
    }
}
