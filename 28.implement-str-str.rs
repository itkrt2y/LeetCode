/*
 * @lc app=leetcode id=28 lang=rust
 *
 * [28] Implement strStr()
 */

// @lc code=start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let haystack_bytes = haystack.as_bytes();
        let haystack_len = haystack.len();
        let needle_bytes = needle.as_bytes();
        let needle_len = needle.len();

        for i in 0..haystack_len {
            let end = i + needle_len;
            if end > haystack_len {
                break;
            }

            let slice = &haystack_bytes[i..end];
            if slice == needle_bytes {
                return i as i32;
            }
        }

        -1
    }
}
// @lc code=end
