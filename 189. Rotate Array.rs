https://leetcode.com/problems/rotate-array/?envType=study-plan-v2&envId=top-interview-150

Sol:
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n; // Handle cases where k > n
        nums.rotate_right(k);
    }
}
