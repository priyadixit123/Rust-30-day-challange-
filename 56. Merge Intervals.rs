56. Merge Intervals
https://leetcode.com/problems/merge-intervals/description/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        // Sort intervals by their start times
        intervals.sort_by_key(|x| x[0]);
        let mut merged: Vec<Vec<i32>> = Vec::new();

        for interval in intervals {
            if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
                // No overlap, push the interval
                merged.push(interval);
            } else {
                // Overlap exists, merge by updating the end
                merged.last_mut().unwrap()[1] = 
                    merged.last().unwrap()[1].max(interval[1]);
            }
        }

        merged
    }
}
