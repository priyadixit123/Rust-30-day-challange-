Pro:  https://leetcode.com/problems/product-of-array-except-self/submissions/1578597372/?envType=study-plan-v2&envId=top-interview-150


Sol:
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![1; n];

        // Compute prefix product
        let mut prefix = 1;
        for i in 0..n {
            answer[i] = prefix;
            prefix *= nums[i];
        }

        // Compute suffix product and multiply with prefix product
        let mut suffix = 1;
        for i in (0..n).rev() {
            answer[i] *= suffix;
            suffix *= nums[i];
        }

        answer
    }
}
