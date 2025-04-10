https://leetcode.com/problems/set-matrix-zeroes/description/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut first_row_has_zero = false;
        let mut first_col_has_zero = false;

        // Check if first row has any zero
        for col in 0..cols {
            if matrix[0][col] == 0 {
                first_row_has_zero = true;
                break;
            }
        }

        // Check if first column has any zero
        for row in 0..rows {
            if matrix[row][0] == 0 {
                first_col_has_zero = true;
                break;
            }
        }

        // Use first row and column to mark zeros
        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][col] == 0 {
                    matrix[row][0] = 0;
                    matrix[0][col] = 0;
                }
            }
        }

        // Set matrix cells to zero based on markers
        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][0] == 0 || matrix[0][col] == 0 {
                    matrix[row][col] = 0;
                }
            }
        }

        // Update first row if needed
        if first_row_has_zero {
            for col in 0..cols {
                matrix[0][col] = 0;
            }
        }

        // Update first column if needed
        if first_col_has_zero {
            for row in 0..rows {
                matrix[row][0] = 0;
            }
        }
    }
}
