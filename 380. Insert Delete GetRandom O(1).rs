Problem
  https://leetcode.com/problems/insert-delete-getrandom-o1/description/?envType=study-plan-v2&envId=top-interview-150
  
  
  Sol:
use rand::prelude::*;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>, // Stores value to index mapping
    list: Vec<i32>,           // Stores values for O(1) random access
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            list: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.list.push(val);
        self.map.insert(val, self.list.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.map.get(&val) {
            let last_val = *self.list.last().unwrap(); // Get last element
            self.list[idx] = last_val; // Swap last element with the one to remove
            self.map.insert(last_val, idx); // Update last element's index
            self.list.pop(); // Remove last element
            self.map.remove(&val); // Remove from map
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let idx = rng.gen_range(0..self.list.len());
        self.list[idx]
    }
}
