/*
 * @lc app=leetcode id=189 lang=rust
 *
 * [189] Rotate Array
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..k {
            let num = nums.pop().unwrap();
            nums.insert(0, num);
        }
    }
}
// @lc code=end
