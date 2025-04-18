Problem:

https://leetcode.com/problems/insert-interval/submissions/1610278316/?envType=study-plan-v2&envId=top-interview-150

Sol:

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut new_interval = new_interval;
        let mut inserted = false;

        for interval in intervals {
            if interval[1] < new_interval[0] {
                // Current interval ends before new interval starts → no overlap
                result.push(interval);
            } else if interval[0] > new_interval[1] {
                // Current interval starts after new interval ends → insert new_interval once
                if !inserted {
                    result.push(new_interval.clone());
                    inserted = true;
                }
                result.push(interval);
            } else {
                // Overlapping intervals → merge them
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        }

        // If new_interval was never inserted (e.g., it belongs at the end)
        if !inserted {
            result.push(new_interval);
        }

        result
    }
}
