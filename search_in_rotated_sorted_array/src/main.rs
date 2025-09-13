fn main() {
    let nums = vec![4,5,6,7,0,1,2];
    let target = 0;

    println!("Result: {}", Solution::search(nums, target));
}

struct Solution;

fn search_recursive(arr: Vec<i32>, left: usize, right: usize, target: i32) -> i32 {
    if left > right {
        return -1;
    }

    let mid = (left + right) / 2;

    if arr[mid] == target {
        return mid as i32;
    }

    // Check if left half is sorted
    if arr[left] <= arr[mid] {
        // Target is in the sorted left half
        if target >= arr[left] && target < arr[mid] {
            search_recursive(arr, left, mid - 1, target)
        } else {
            search_recursive(arr, mid + 1, right, target)
        }
    } else {
        // Right half is sorted
        // Target is in the sorted right half
        if target > arr[mid] && target <= arr[right] {
            search_recursive(arr, mid + 1, right, target)
        } else {
            search_recursive(arr, left, mid - 1, target)
        }
    }
}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let left = 0;
        let right = nums.len() - 1;

        search_recursive(nums, left, right, target)
    }
}