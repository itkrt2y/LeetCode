/*
 * @lc app=leetcode id=58 lang=rust
 *
 * [58] Length of Last Word
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut words = s.split(" ").collect::<Vec<&str>>();
        loop {
            match words.last() {
                None => return 0,
                Some(&"") => {
                    words.pop().unwrap();
                }
                Some(word) => return word.len() as i32,
            }
        }
    }
}
// @lc code=end
