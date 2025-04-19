https://leetcode.com/problems/min-stack/description/?envType=study-plan-v2&envId=top-interview-150


Sol:

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = if let Some(&last_min) = self.min_stack.last() {
            val.min(last_min)
        } else {
            val
        };
        self.min_stack.push(min);
    }

    fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}
