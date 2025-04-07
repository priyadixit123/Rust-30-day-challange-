Problem :
https://leetcode.com/problems/max-number-of-k-sum-pairs/description/

Sol

use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut count = 0;

        for &num in &nums {
            let complement = k - num;
            if let Some(&val) = map.get(&complement) {
                if val > 0 {
                    count += 1;
                    map.insert(complement, val - 1);
                } else {
                    *map.entry(num).or_insert(0) += 1;
                }
            } else {
                *map.entry(num).or_insert(0) += 1;
            }
        }

        count
    }
}
