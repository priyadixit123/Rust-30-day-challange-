https://leetcode.com/problems/simplify-path/description/?envType=study-plan-v2&envId=top-interview-150


Sol

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();

        for part in path.split('/') {
            match part {
                "" | "." => continue, // Skip empty or current directory
                ".." => { stack.pop(); }, // Go up one level
                _ => stack.push(part), // Valid directory name
            }
        }

        // Join the simplified parts with a slash and ensure it starts with '/'
        format!("/{}", stack.join("/"))
    }
}
