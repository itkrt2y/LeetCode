/*
 * @lc app=leetcode id=155 lang=rust
 *
 * [155] Min Stack
 */

// @lc code=start
struct MinStack {
    stack: Vec<i32>,
    min: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack: vec![],
            min: i32::max_value(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.min >= x {
            self.stack.push(self.min);
            self.min = x;
        }
        self.stack.push(x)
    }

    fn pop(&mut self) {
        let num = self.stack.pop().unwrap();
        if self.min == num {
            self.min = self.stack.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        self.min
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @lc code=end

