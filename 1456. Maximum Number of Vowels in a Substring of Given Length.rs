1456. Maximum Number of Vowels in a Substring of Given Length

https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/description/?envType=study-plan-v2&envId=leetcode-75

Sol:

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowels = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
        let (mut count, mut max_count) = (0, 0);
        let s_chars: Vec<char> = s.chars().collect();
        let k = k as usize;

        for i in 0..s_chars.len() {
            if vowels(s_chars[i]) {
                count += 1;
            }

            if i >= k {
                if vowels(s_chars[i - k]) {
                    count -= 1;
                }
            }

            max_count = max_count.max(count);
        }

        max_count
    }
}
