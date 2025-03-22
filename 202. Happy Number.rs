https://leetcode.com/problems/happy-number/submissions/1582502011/?envType=study-plan-v2&envId=top-interview-150

Sol:
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();

        while n != 1 {
            if seen.contains(&n) {
                return false;
            }
            seen.insert(n);
            n = Self::sum_of_squares(n);
        }

        true
    }

    fn sum_of_squares(mut n: i32) -> i32 {
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}
