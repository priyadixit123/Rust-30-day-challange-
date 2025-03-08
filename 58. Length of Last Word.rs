https://leetcode.com/problems/length-of-last-word/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split_whitespace().last().unwrap().len() as i32
    }
}
