https://leetcode.com/problems/gas-station/submissions/1576207008/?envType=study-plan-v2&envId=top-interview-150


Sol:


impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_tank = 0;
        let mut current_tank = 0;
        let mut start_index = 0;

        for i in 0..gas.len() {
            let balance = gas[i] - cost[i];
            total_tank += balance;
            current_tank += balance;
            
            if current_tank < 0 {
                start_index = i as i32 + 1;
                current_tank = 0;
            }
        }

        if total_tank >= 0 {
            start_index
        } else {
            -1
        }
    }
}
l:
