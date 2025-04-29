https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/?envType=daily-question&envId=2025-04-29

Sol:


impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max_val = *nums.iter().max().unwrap();
        let mut ans = 0i64;
        let mut left = 0;
        let mut count = 0;
        
        for right in 0..nums.len() {
            if nums[right] == max_val {
                count += 1;
            }
            while count >= k {
                if nums[left] == max_val {
                    count -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }
        
        ans
    }
}
