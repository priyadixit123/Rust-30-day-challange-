Problem : 
https://leetcode.com/problems/summary-ranges/description/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < nums.len() {
            let start = nums[i];

            while i + 1 < nums.len() && nums[i] + 1 == nums[i + 1] {
                i += 1;
            }

            if start == nums[i] {
                result.push(start.to_string());
            } else {
                result.push(format!("{}->{}", start, nums[i]));
            }

            i += 1;
        }

        result
    }
}
