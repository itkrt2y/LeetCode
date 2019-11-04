/*
 * @lc app=leetcode id=171 lang=rust
 *
 * [171] Excel Sheet Column Number
 */

// @lc code=start
impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut result: i32 = 0;
        let bytes = s.as_bytes();
        let a_byte_code = 'A' as i32;

        for i in 0..s.len() {
            result = result * 26 + (bytes[i] as i32) - a_byte_code + 1;
        }

        result
    }
}
// @lc code=end
