https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&envId=top-interview-150


Sol:
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order
        let mut h = 0;

        for (i, &c) in citations.iter().enumerate() {
            if c >= (i as i32 + 1) {
                h = i as i32 + 1;
            } else {
                break;
            }
        }

        h
    }
}
