use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut max_heap = BinaryHeap::new();

        let scores_with_index = score.into_iter()
            .enumerate()
            .for_each(|(index, item)| {
                println!("{:?} - {:?}", index, item);
                max_heap.push((item, index));
            });

        println!("---------------------------------------------");

        // while let Some(val) = max_heap.pop() {
        //     println!("{:?}", val);
        // }
        //
        max_heap.iter().for_each(|element| println!("{:?}", element));

        let mut result_vector_tuples: Vec<(i32, usize, usize)> = max_heap
            .into_iter()
            .enumerate()
            .map(|(mut place, (value, index))| (value, place + 1, index))
            .collect();

        result_vector_tuples.sort_by(|(_, _, a_index), (_, _, b_index)| a_index.cmp(b_index));

        println!("{:?}", result_vector_tuples);

        let resulting_array_with_strings: Vec<String> = result_vector_tuples.into_iter().map(|(value, place, index)| {
            get_string_based_on_place(place)
        })
            .collect();

        resulting_array_with_strings
    }

    pub fn find_relative_ranks_chat_gpt(score: Vec<i32>) -> Vec<String> {
        let mut max_heap = BinaryHeap::new();

        for (index, item) in score.iter().enumerate() {
            max_heap.push((*item, index)); // Push by value, not reference
        }

        let mut result_vector_tuples: Vec<(usize, usize)> = vec![(0, 0); score.len()];

        for place in 1..=score.len() {
            if let Some((_, index)) = max_heap.pop() {
                result_vector_tuples[index] = (place, index);
            }
        }

        result_vector_tuples.sort_by_key(|&(_, index)| index);

        result_vector_tuples
            .into_iter()
            .map(|(place, _)| get_string_based_on_place(place))
            .collect()
    }
}

fn get_string_based_on_place(place: usize) -> String {
    match place {
        1 => String::from("Gold Medal"),
        2 => String::from("Silver Medal"),
        3 => String::from("Bronze Medal"),
        not_medal_place => format!("{}", not_medal_place)
    }
}

fn main() {

    let scores_vector = vec![10,3,8,9,4];

    let result = Solution::find_relative_ranks_chat_gpt(scores_vector);

    println!("{:?}", result);
}
