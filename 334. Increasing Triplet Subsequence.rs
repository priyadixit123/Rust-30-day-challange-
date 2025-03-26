https://leetcode.com/problems/increasing-triplet-subsequence/submissions/1586686839/?envType=study-plan-v2&envId=leetcode-75


Sol:
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first, mut second) = (i32::MAX, i32::MAX);

        for &num in &nums {
            if num <= first {
                first = num; // Smallest number so far
            } else if num <= second {
                second = num; // Second smallest number so far
            } else {
                return true; // Found a triplet
            }
        }

        false
    }
}
