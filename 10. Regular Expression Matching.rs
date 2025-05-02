https://leetcode.com/problems/regular-expression-matching/description/

Sol:

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        
        // DP table where dp[i][j] means s[0..i] matches p[0..j]
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        
        // Base case: Empty string matches empty pattern
        dp[0][0] = true;
        
        // Initialize the first row (when s is empty)
        for j in 1..=p.len() {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];  // Match '*' as zero occurrence of the preceding character
            }
        }

        // Fill the DP table
        for i in 1..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == b'.' || s[i - 1] == p[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];  // If characters match or '.' is used
                } else if p[j - 1] == b'*' {
                    dp[i][j] = dp[i][j - 2] || (dp[i - 1][j] && (s[i - 1] == p[j - 2] || p[j - 2] == b'.'));
                }
            }
        }

        dp[s.len()][p.len()]  // The result is whether the whole string matches the whole pattern
    }
}
