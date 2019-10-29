/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut words = s.trim().split(" ").collect::<Vec<&str>>();
        match words.pop() {
            None => 0,
            Some(word) => word.len() as i32,
        }
    }
}
// @lc code=end
