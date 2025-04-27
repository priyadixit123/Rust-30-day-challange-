https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/description/?envType=daily-question&envId=2025-04-27


Sol:
 impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len().saturating_sub(2) {
            let a = nums[i];
            let b = nums[i + 1];
            let c = nums[i + 2];
            if (a + c) * 2 == b {
                count += 1;
            }
        }
        count
    }
}
