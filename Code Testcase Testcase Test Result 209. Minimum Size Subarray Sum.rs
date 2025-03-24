https://leetcode.com/problems/minimum-size-subarray-sum/submissions/1584100861/?envType=study-plan-v2&envId=top-interview-150


Sol:
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut left, mut sum) = (0, 0);
        let mut min_length = i32::MAX;

        for right in 0..nums.len() {
            sum += nums[right];

            while sum >= target {
                min_length = min_length.min((right - left + 1) as i32);
                sum -= nums[left];
                left += 1;
            }
        }

        if min_length == i32::MAX { 0 } else { min_length }
    }
}
