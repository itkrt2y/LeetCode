/*
 * @lc app=leetcode id=167 lang=rust
 *
 * [167] Two Sum II - Input array is sorted
 */

// @lc code=start
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, num) in numbers.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(&j) => return vec![j as i32 + 1, i as i32 + 1],
                None => map.insert(num, i),
            };
        }

        vec![]
    }
}
// @lc code=end
