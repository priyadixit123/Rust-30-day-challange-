Problem: https://leetcode.com/problems/can-place-flowers/?envType=study-plan-v2&envId=leetcode-75

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty, and an integer n, 
return true if n new flowers can be planted in the flowerbed without violating the no-adjacent-flowers rule and false otherwise.

Solution 

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed;
        let mut count = 0;
        let len = flowerbed.len();
        
        for i in 0..len {
            if flowerbed[i] == 0 {
                let left_empty = i == 0 || flowerbed[i - 1] == 0;
                let right_empty = i == len - 1 || flowerbed[i + 1] == 0;
                
                if left_empty && right_empty {
                    flowerbed[i] = 1;
                    count += 1;
                    if count >= n {
                        return true;
                    }
                }
            }
        }
        
        count >= n
    }
}
