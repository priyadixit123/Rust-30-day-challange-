https://leetcode.com/problems/string-to-integer-atoi/description/


Sol:


impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_start(); // Step 1: Trim leading whitespace
        if s.is_empty() {
            return 0;
        }

        let mut sign = 1;
        let mut index = 0;
        let chars: Vec<char> = s.chars().collect();

        // Step 2: Handle sign
        if chars[index] == '-' {
            sign = -1;
            index += 1;
        } else if chars[index] == '+' {
            index += 1;
        }

        let mut result: i64 = 0; // Use i64 to avoid overflow

        // Step 3: Parse digits
        while index < chars.len() && chars[index].is_ascii_digit() {
            result = result * 10 + (chars[index] as i64 - '0' as i64);

            // Step 4: Clamp if out of bounds
            if sign * result > i32::MAX as i64 {
                return i32::MAX;
            } else if sign * result < i32::MIN as i64 {
                return i32::MIN;
            }

            index += 1;
        }

        (sign * result) as i32
    }
}
