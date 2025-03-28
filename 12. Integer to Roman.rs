https://leetcode.com/problems/integer-to-roman/submissions/1588743545/?envType=study-plan-v2&envId=top-interview-150

Sol:
impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let symbols = [
            (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
            (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
            (10, "X"), (9, "IX"), (5, "V"), (4, "IV"),
            (1, "I")
        ];
        
        let mut result = String::new();
        
        for &(value, symbol) in &symbols {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }
        
        result
    }
}
