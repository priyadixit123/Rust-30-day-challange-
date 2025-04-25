https://leetcode.com/problems/count-of-interesting-subarrays/description/?envType=daily-question&envId=2025-04-25


Sol:

use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let mut prefix = 0;
        let mut count_map: HashMap<i32, i64> = HashMap::new();
        count_map.insert(0, 1); // Initialize for empty prefix
        let mut result = 0;

        for num in nums {
            if num % modulo == k {
                prefix = (prefix + 1) % modulo;
            } else {
                prefix = prefix % modulo;
            }

            // Calculate (prefix - k + modulo) % modulo to avoid negative mod
            let needed = (prefix - k + modulo) % modulo;

            if let Some(&cnt) = count_map.get(&needed) {
                result += cnt;
            }

            *count_map.entry(prefix).or_insert(0) += 1;
        }

        result
    }
}
