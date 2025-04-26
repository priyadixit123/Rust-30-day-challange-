https://leetcode.com/problems/count-subarrays-with-fixed-bounds/description/?envType=daily-question&envId=2025-04-26


Sol:
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0i64;
        let mut min_index = -1;  // Last index where min_k was seen
        let mut max_index = -1;  // Last index where max_k was seen
        let mut left_bound = -1; // Last index where an invalid number was seen

        for (i, &num) in nums.iter().enumerate() {
            if num < min_k || num > max_k {
                left_bound = i as i32;
            }
            if num == min_k {
                min_index = i as i32;
            }
            if num == max_k {
                max_index = i as i32;
            }

            let valid_start = min_index.min(max_index);
            if valid_start > left_bound {
                result += (valid_start - left_bound) as i64;
            }
        }

        result
    }
}
