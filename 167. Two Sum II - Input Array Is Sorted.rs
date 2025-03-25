https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/?envType=study-plan-v2&envId=top-interview-150



Sol:
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);

        while left < right {
            let sum = numbers[left] + numbers[right];

            if sum == target {
                return vec![(left + 1) as i32, (right + 1) as i32]; // Convert to 1-based index
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        vec![]
    }
}
