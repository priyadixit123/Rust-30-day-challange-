https://leetcode.com/problems/ransom-note/description/?envType=study-plan-v2&envId=top-interview-150

383. Ransom Note

Sol:

use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq = HashMap::new();

        for ch in magazine.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }

        for ch in ransom_note.chars() {
            if let Some(count) = freq.get_mut(&ch) {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            } else {
                return false;
            }
        }

        true
    }
}

Explain : https://blocksimplifier.com/383-ransom-note-leetcode-rust-solution/
