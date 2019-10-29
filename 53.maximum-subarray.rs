/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

// @lc code=start
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_so_far = nums[0];
        let mut max_ending_here = nums[0];

        for num in nums[1..].iter() {
            max_ending_here = max(max_ending_here + num, *num);
            max_so_far = max(max_so_far, max_ending_here);
        }

        max_so_far
    }
}
// @lc code=end
