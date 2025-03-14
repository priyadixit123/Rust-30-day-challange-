https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/submissions/1573074136/?envType=study-plan-v2&envId=top-interview-150


Sol:
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max_profit += prices[i] - prices[i - 1];
            }
        }
        
        max_profit
    }
}
