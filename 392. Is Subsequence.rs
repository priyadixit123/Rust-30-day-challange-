https://leetcode.com/problems/is-subsequence/submissions/1568630126/?envType=study-plan-v2&envId=top-interview-150

Sol:
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut s_iter = s_chars.next();

        for c in t.chars() {
            if let Some(sc) = s_iter {
                if sc == c {
                    s_iter = s_chars.next();
                }
            } else {
                return true;
            }
        }

        s_iter.is_none()
    }
}
