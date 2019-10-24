/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(&index) => {
                    return vec![index as i32, i as i32];
                },
                None => map.insert(num, i),
            };
        }
        vec![]
    }
}
// @lc code=end

