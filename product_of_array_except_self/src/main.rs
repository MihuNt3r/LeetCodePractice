fn main() {
    let vector = vec![1,2,3,4];

    let result = Solution::product_except_self(vector);

    println!("Result vector: {:?}", result);
}

struct Solution;

impl Solution {
    pub fn product_except_self_filter_map(nums: Vec<i32>) -> Vec<i32> {
        let result = nums
            .iter()
            .enumerate()
            .map(|(index, num)| {
                nums.iter()
                    .enumerate()
                    .filter_map(|(index_inner, &num)| {
                        if index != index_inner {
                            Some(num)
                        }  else {
                            None
                        }
                    })
                    .product()
            })
            .collect();

        result
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let result = nums
            .iter()
            .enumerate()
            .map(|(index, num)| {
                nums.iter()
                    .enumerate()
                    .filter(|(index_inner, _)| index != *index_inner)
                    .map(|(_, &number)| number)
                    .product()
            })
            .collect();

        result
    }
}
