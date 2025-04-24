https://leetcode.com/problems/count-complete-subarrays-in-an-array/description/?envType=daily-question&envId=2025-04-24



Sol:
use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let total_unique: usize = nums.iter().collect::<HashSet<_>>().len();
        let mut count = 0;
        let n = nums.len();

        for i in 0..n {
            let mut freq: HashMap<i32, i32> = HashMap::new();
            for j in i..n {
                *freq.entry(nums[j]).or_insert(0) += 1;
                if freq.len() == total_unique {
                    count += 1;
                }
            }
        }

        count
    }
}
