/*
 * @lc app=leetcode id=172 lang=rust
 *
 * [172] Factorial Trailing Zeroes
 */

// @lc code=start
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut n = n;
        let mut result = 0;

        while n > 0 {
            n /= 5;
            result += n;
        }

        result
    }
}
// @lc code=end
