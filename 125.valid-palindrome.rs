/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars().filter(|c| c.is_ascii_alphanumeric());
        chars
            .clone()
            .collect::<String>()
            .eq_ignore_ascii_case(&chars.rev().collect::<String>())
    }
}
// @lc code=end
