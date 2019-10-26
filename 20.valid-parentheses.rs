/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut iter = s.chars();
        let mut expects = vec![];

        loop {
            match iter.next() {
                None => return expects.is_empty(),
                Some(c) => match c {
                    '(' => expects.push(')'),
                    ')' => match expects.pop() {
                        Some(')') => {}
                        _ => return false,
                    },

                    '{' => expects.push('}'),
                    '}' => match expects.pop() {
                        Some('}') => {}
                        _ => return false,
                    },

                    '[' => expects.push(']'),
                    ']' => match expects.pop() {
                        Some(']') => {}
                        _ => return false,
                    },
                    _ => return false,
                },
            }
        }
    }
}
// @lc code=end
