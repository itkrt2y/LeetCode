/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut vec = vec![];

        let mut carry = 0;

        for i in 0..digits.len() {
            let j = digits.len() - 1 - i;
            let num = if i == 0 {
                digits[j] + 1
            } else {
                digits[j] + carry
            };

            if num < 10 {
                vec.push(num);
                carry = 0;
            } else {
                vec.push(num - 10);
                carry = 1;
            }
        }

        if carry == 1 {
            vec.push(1);
        }

        vec.reverse();
        vec
    }
}
// @lc code=end
