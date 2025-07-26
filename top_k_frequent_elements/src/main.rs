use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    let vector = vec![1,1,1,2,2,3];

    let solution_vec = Solution::top_k_frequent(vector, 2);

    println!("Hello, world! {:?}", solution_vec);
}

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash_map = HashMap::new();

        nums.iter().for_each(|num| {
            let already_existing_value = hash_map.get_mut(num);

            match already_existing_value {
                Some(count) => *count += 1,
                None => { hash_map.insert(num, 1); }
            }
        });

        println!("{:?}", hash_map);

        let mut max_heap = BinaryHeap::new();

        hash_map.iter().for_each(|(&number, frequency)| {
            let tuple = (frequency, number);

            max_heap.push(tuple);
        });

        println!("{:?}", max_heap);

        let mut result_vector = vec![];

        for i in 0..k {
            let value = max_heap.pop();

            match value {
                Some((_, number)) => result_vector.push(*number),
                None => {}
            }
        }

        result_vector
    }
}
