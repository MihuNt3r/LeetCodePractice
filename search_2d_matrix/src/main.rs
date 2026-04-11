fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let m = matrix.len();
        let n = matrix[0].len();

        // Step 1: Binary search to find the correct row
        let mut top = 0;
        let mut bottom = m - 1;

        while top <= bottom {
            let mid = top + (bottom - top) / 2;

            if matrix[mid][0] > target {
                bottom = mid - 1;
            } else if matrix[mid][n - 1] < target {
                top = mid + 1;
            } else {
                // Target is in this row (or could be)
                return Self::binary_search_row(&matrix[mid], target);
            }
        }

        false
    }

    fn binary_search_row(row: &[i32], target: i32) -> bool {
        let mut left = 0;
        let mut right = row.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if row[mid] == target {
                return true;
            } else if row[mid] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}
