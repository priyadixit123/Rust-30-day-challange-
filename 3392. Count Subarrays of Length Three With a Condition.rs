https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/description/?envType=daily-question&envId=2025-04-27


Sol:
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() - 2 {
            if nums[i] + nums[i + 2] == nums[i + 1] / 2 {
                count += 1;
            }
        }
        count
    }
}
