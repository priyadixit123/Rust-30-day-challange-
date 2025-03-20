


https://leetcode.com/problems/isomorphic-strings/submissions/1579686178/?envType=study-plan-v2&envId=top-interview-150


Sol:

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map_s = HashMap::new();
        let mut map_t = HashMap::new();
        
        let chars_s: Vec<char> = s.chars().collect();
        let chars_t: Vec<char> = t.chars().collect();
        
        for i in 0..chars_s.len() {
            let ch_s = chars_s[i];
            let ch_t = chars_t[i];

            if map_s.get(&ch_s) != map_t.get(&ch_t) {
                return false;
            }

            map_s.insert(ch_s, i);
            map_t.insert(ch_t, i);
        }
        
        true
    }
}
