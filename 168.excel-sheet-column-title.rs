/*
 * @lc app=leetcode id=168 lang=rust
 *
 * [168] Excel Sheet Column Title
 */

// @lc code=start
impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut n = n;
        let mut result = "".to_string();

        let base = 'A' as u8;
        while n > 0 {
            n -= 1;
            result.insert(0, (base + (n % 26) as u8) as char);
            n /= 26;
        }

        result
    }
}
// @lc code=end
