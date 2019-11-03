/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        match row_index {
            0 => return vec![1],
            _ => {}
        }

        let mut triangle = vec![vec![1]];

        for i in 0..row_index {
            let mut row = vec![1];
            let mut iter = triangle.last().unwrap().iter().peekable();
            loop {
                match (iter.next(), iter.peek()) {
                    (Some(current), Some(next)) => row.push(current + *next),
                    _ => break,
                }
            }
            row.push(1);

            if i == row_index - 1 {
                return row;
            }

            triangle.push(row);
        }

        vec![]
    }
}
// @lc code=end
