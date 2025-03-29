Problem:
https://leetcode.com/problems/valid-sudoku/description/?envType=study-plan-v2&envId=top-interview-150



Sol:

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new(); 9];
        let mut cols = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];

        for i in 0..9 {
            for j in 0..9 {
                let cell = board[i][j];
                if cell == '.' {
                    continue;
                }

                let box_index = (i / 3) * 3 + (j / 3);
                
                if rows[i].contains(&cell) || cols[j].contains(&cell) || boxes[box_index].contains(&cell) {
                    return false;
                }

                rows[i].insert(cell);
                cols[j].insert(cell);
                boxes[box_index].insert(cell);
            }
        }

        true
    }
}
