https://leetcode.com/problems/longest-consecutive-sequence/description/?envType=study-plan-v2&envId=top-interview-150

Sol:

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;

        for &num in &num_set {
            // Only start counting if it's the beginning of a sequence
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }

                longest = longest.max(current_streak);
            }
        }

        longest
    }
}
