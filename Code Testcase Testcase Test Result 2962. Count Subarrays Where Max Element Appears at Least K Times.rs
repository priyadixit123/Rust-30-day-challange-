https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/?envType=daily-question&envId=2025-04-29

Sol:


impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut count = 0i64;
        let max_val = *nums.iter().max().unwrap();
        
        let n = nums.len();
        
        for i in 0..n {
            let mut max_count = 0;
            for j in i..n {
                if nums[j] == max_val {
                    max_count += 1;
                }
                if max_count >= k {
                    count += 1;
                }
            }
        }
        
        count
    }
}
