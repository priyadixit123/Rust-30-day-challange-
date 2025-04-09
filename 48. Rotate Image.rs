Problem:

https://leetcode.com/problems/rotate-image/description/?envType=study-plan-v2&envId=top-interview-150

Sol:
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        // Step 1: Transpose the matrix
        for i in 0..n {
            for j in i+1..n {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        // Step 2: Reverse each row
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}


