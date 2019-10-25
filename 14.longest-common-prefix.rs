/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }

        let mut prefix = String::from("");
        let mut i = 0;
        loop {
            let mut c = None;
            for word in &strs {
                if i == word.len() {
                    return prefix;
                }

                match c {
                    None => c = word.chars().nth(i),
                    Some(c) => match word.chars().nth(i) {
                        Some(c2) if c == c2 => continue,
                        _ => return prefix,
                    },
                }
            }
            if let Some(ch) = c {
                prefix.push(ch);
            }

            i += 1;
        }
    }
}
// @lc code=end
