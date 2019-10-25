/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut iter = s.chars().peekable();
        let mut num = 0;

        loop {
            match iter.next() {
                Some(c) => match c {
                    'I' => match iter.peek() {
                        Some('V') => {
                            num += 4;
                            iter.next();
                        }
                        Some('X') => {
                            num += 9;
                            iter.next();
                        }
                        _ => num += 1,
                    },
                    'X' => match iter.peek() {
                        Some('L') => {
                            num += 40;
                            iter.next();
                        }
                        Some('C') => {
                            num += 90;
                            iter.next();
                        }
                        _ => num += 10,
                    },
                    'C' => match iter.peek() {
                        Some('D') => {
                            num += 400;
                            iter.next();
                        }
                        Some('M') => {
                            num += 900;
                            iter.next();
                        }
                        _ => num += 100,
                    },
                    'V' => num += 5,
                    'L' => num += 50,
                    'D' => num += 500,
                    'M' => num += 1000,
                    _ => {}
                },
                None => break,
            }
        }

        num
    }
}
// @lc code=end
