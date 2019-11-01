/*
 * @lc app=leetcode id=88 lang=rust
 *
 * [88] Merge Sorted Array
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let k = m + n - 1;

        while (j as i32) < n {
            if nums1[i] > nums2[j] {
                nums1.pop();
                nums1.insert(i, nums2[j]);
                i += 1;
                j += 1;
            } else {
                if i >= m as usize + j {
                    nums1.pop();
                    nums1.insert(i, nums2[j]);
                    j += 1;
                }

                if i < k as usize {
                    i += 1;
                }
            }
        }
    }
}
// @lc code=end
