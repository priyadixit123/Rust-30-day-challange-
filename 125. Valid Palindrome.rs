https://leetcode.com/problems/valid-palindrome/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let filtered: String = s.chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();
        
        filtered.chars().eq(filtered.chars().rev())
    }
}
