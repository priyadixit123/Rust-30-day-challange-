Problem : https://leetcode.com/problems/valid-anagram/?envType=study-plan-v2&envId=top-interview-150

Sol:
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        
        s_chars.sort_unstable();
        t_chars.sort_unstable();
        
        s_chars == t_chars
    }
}
