/*
 * @lc app=leetcode id=67 lang=rust
 *
 * [67] Add Binary
 */

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a;
        let mut b = b;
        let mut result = String::from("");
        let mut carry = false;

        loop {
            match (a.pop(), b.pop()) {
                (Some('1'), Some('1')) => {
                    result.insert(0, if carry { '1' } else { '0' });
                    carry = true;
                }

                (Some('1'), Some('0'))
                | (Some('0'), Some('1'))
                | (Some('1'), None)
                | (None, Some('1')) => {
                    if carry {
                        result.insert(0, '0');
                        carry = true;
                    } else {
                        result.insert(0, '1');
                        carry = false;
                    }
                }
                (Some('0'), Some('0')) | (Some('0'), None) | (None, Some('0')) => {
                    result.insert(0, if carry { '1' } else { '0' });
                    carry = false;
                }

                _ => break,
            }
        }

        if carry {
            result.insert(0, '1')
        };

        result
    }
}
// @lc code=end
