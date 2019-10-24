/*
 * @lc app=leetcode id=1 lang=javascript
 *
 * [1] Two Sum
 */

// @lc code=start
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
function twoSum(nums, target) {
  const map = new Map();

  for (let i = 0; i < nums.length; i++) {
    const cur = nums[i];
    const complementIndex = map.get(target - cur);
    if (complementIndex !== undefined) {
      return [complementIndex, i];
    }

    map.set(cur, i);
  }
}
// @lc code=end
