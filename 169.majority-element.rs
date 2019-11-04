/*
 * @lc app=leetcode id=169 lang=rust
 *
 * [169] Majority Element
 */

// @lc code=start
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        nums[nums.len() / 2]
    }
}
// @lc code=end
