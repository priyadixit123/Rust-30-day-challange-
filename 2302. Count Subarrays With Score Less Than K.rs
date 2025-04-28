https://leetcode.com/problems/count-subarrays-with-score-less-than-k/description/?envType=daily-question&envId=2025-04-28

Sol:

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        let mut left = 0;
        let mut sum = 0i64;
        let mut ans = 0i64;

        for right in 0..nums.len() {
            sum += nums[right] as i64;

            while sum * (right as i64 - left as i64 + 1) >= k {
                sum -= nums[left] as i64;
                left += 1;
            }

            ans += (right - left + 1) as i64;
        }

        ans
    }
}
