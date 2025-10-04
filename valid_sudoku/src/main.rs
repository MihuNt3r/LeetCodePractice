use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let n = 9;

        let mut rows: Vec<HashSet<char>> = Vec::new();
        let mut columns: Vec<HashSet<char>> = Vec::new();
        let mut boxes: Vec<HashSet<char>> = Vec::new();

        for _n in 0..n {
            rows.push(HashSet::with_capacity(n));
            columns.push(HashSet::with_capacity(n));
            boxes.push(HashSet::with_capacity(n));
        }

        for r in 0..n {
            for c in 0..n {
                let val = board[r][c];

                if val == '.' {
                    continue;
                }

                if rows[r].contains(&val) {
                    return false;
                }
                rows[r].insert(val);

                if columns[c].contains(&val) {
                    return false;
                }
                columns[c].insert(val);

                let idx = (r / 3) * 3 + (c / 3);
                if boxes[idx].contains(&val) {
                    return false;
                }
                boxes[idx].insert(val);
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
}
