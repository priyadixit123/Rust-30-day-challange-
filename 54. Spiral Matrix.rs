Problem:
https://leetcode.com/problems/spiral-matrix/?envType=study-plan-v2&envId=top-interview-150

Sol:


impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return vec![];
        }

        let (mut top, mut bottom) = (0, matrix.len() as i32 - 1);
        let (mut left, mut right) = (0, matrix[0].len() as i32 - 1);
        let mut result = Vec::new();

        while top <= bottom && left <= right {
            // Traverse from Left to Right
            for col in left..=right {
                result.push(matrix[top as usize][col as usize]);
            }
            top += 1;

            // Traverse from Top to Bottom
            for row in top..=bottom {
                result.push(matrix[row as usize][right as usize]);
            }
            right -= 1;

            // Traverse from Right to Left
            if top <= bottom {
                for col in (left..=right).rev() {
                    result.push(matrix[bottom as usize][col as usize]);
                }
                bottom -= 1;
            }

            // Traverse from Bottom to Top
            if left <= right {
                for row in (top..=bottom).rev() {
                    result.push(matrix[row as usize][left as usize]);
                }
                left += 1;
            }
        }

        result
    }
}
