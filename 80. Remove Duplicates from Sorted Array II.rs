https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/submissions/1571221027/?envType=study-plan-v2&envId=top-interview-150



Sol:

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        
        // Position to place the next valid number
        let mut k = 2;
        
        // Start from 3rd element
        for i in 2..nums.len() {
            // If current number is different from number two positions back,
            // we can include it
            if nums[i] != nums[k - 2] {
                nums[k] = nums[i];
                k += 1;
            }
        }
        
        k as i32
    }
}
