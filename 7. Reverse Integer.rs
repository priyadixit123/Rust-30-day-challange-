https://leetcode.com/problems/reverse-integer/description/



Sol

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x = x;
        let mut result = 0;

        while x != 0 {
            let digit = x % 10;
            x /= 10;

            // Check for overflow before multiplying or adding
            if result > i32::MAX / 10 || (result == i32::MAX / 10 && digit > 7) {
                return 0;
            }
            if result < i32::MIN / 10 || (result == i32::MIN / 10 && digit < -8) {
                return 0;
            }

            result = result * 10 + digit;
        }

        result
    }
}
