https://leetcode.com/problems/number-of-equivalent-domino-pairs/description/?envType=daily-question&envId=2025-05-04

Sol:

use std::collections::HashMap;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count = HashMap::new();
        let mut result = 0;

        for domino in dominoes {
            let a = domino[0].min(domino[1]);
            let b = domino[0].max(domino[1]);
            let key = (a, b);
            let entry = count.entry(key).or_insert(0);
            result += *entry;
            *entry += 1;
        }

        result
    }
}
