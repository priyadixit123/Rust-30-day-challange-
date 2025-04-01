Problem :
https://leetcode.com/problems/climbing-stairs/description/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }

        let (mut first, mut second) = (1, 2);
        for _ in 3..=n {
            let third = first + second;
            first = second;
            second = third;
        }

        second
    }
}
