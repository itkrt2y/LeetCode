/*
 * @lc app=leetcode id=35 lang=rust
 *
 * [35] Search Insert Position
 */

// @lc code=start
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for (i, num) in nums.iter().enumerate() {
            if i == 0 && target < *num {
                return 0;
            }
            if *num == target {
                return i as i32;
            }

            match nums.iter().nth(i + 1) {
                None => break,
                Some(next_num) => {
                    if *num < target && target < *next_num {
                        return (i + 1) as i32;
                    }
                }
            }
        }

        nums.len() as i32
    }
}
// @lc code=end
