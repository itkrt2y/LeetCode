/*
 * @lc app=leetcode id=69 lang=rust
 *
 * [69] Sqrt(x)
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;
        let mut r: i64 = x;
        while r * r > x {
            r = (r + x / r) / 2;
        }
        r as i32
    }
}
// @lc code=end
