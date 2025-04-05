Problem 
https://leetcode.com/problems/game-of-life/description/?envType=study-plan-v2&envId=top-interview-150

Sol:
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1),  (1, 0), (1, 1),
        ];
        
        let m = board.len();
        let n = board[0].len();

        // First pass: apply rules and mark changes
        for i in 0..m {
            for j in 0..n {
                let mut live_neighbors = 0;
                for (dx, dy) in &directions {
                    let ni = i as isize + dx;
                    let nj = j as isize + dy;
                    if ni >= 0 && nj >= 0 && ni < m as isize && nj < n as isize {
                        let ni = ni as usize;
                        let nj = nj as usize;
                        if board[ni][nj] == 1 || board[ni][nj] == 2 {
                            live_neighbors += 1;
                        }
                    }
                }

                // Apply rules with temporary states:
                // 2: live -> dead
                // 3: dead -> live
                if board[i][j] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        board[i][j] = 2;
                    }
                } else {
                    if live_neighbors == 3 {
                        board[i][j] = 3;
                    }
                }
            }
        }

        // Second pass: finalize the state
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 2 {
                    board[i][j] = 0;
                } else if board[i][j] == 3 {
                    board[i][j] = 1;
                }
            }
        }
    }
}
