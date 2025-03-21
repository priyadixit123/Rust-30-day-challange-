https://leetcode.com/problems/word-pattern/description/?envType=study-plan-v2&envId=top-interview-150


Sol:
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut char_to_word = HashMap::new();
        let mut word_to_char = HashMap::new();

        for (c, word) in pattern.chars().zip(words.iter()) {
            if let Some(mapped_word) = char_to_word.get(&c) {
                if mapped_word != word {
                    return false;
                }
            } else if let Some(mapped_char) = word_to_char.get(word) {
                if mapped_char != &c {
                    return false;
                }
            } else {
                char_to_word.insert(c, *word);
                word_to_char.insert(*word, c);
            }
        }
        
        true
    }
}
