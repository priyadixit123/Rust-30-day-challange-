Problem :
  
  https://leetcode.com/problems/string-compression/description/?envType=study-plan-v2&envId=leetcode-75
  
  
Sol:

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut write = 0;
        let mut read = 0;

        while read < chars.len() {
            let current_char = chars[read];
            let mut count = 0;

            // Count occurrences of the current character
            while read < chars.len() && chars[read] == current_char {
                read += 1;
                count += 1;
            }

            // Write the character
            chars[write] = current_char;
            write += 1;

            // Write the count if greater than 1
            if count > 1 {
                for digit in count.to_string().chars() {
                    chars[write] = digit;
                    write += 1;
                }
            }
        }

        write as i32
    }
}


 Time and Space:
Time Complexity: O(n)

Space Complexity: O(1) (except for temporary string conversion which is allowed)
