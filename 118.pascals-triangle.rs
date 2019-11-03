/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        match num_rows {
            0 => return vec![],
            1 => return vec![vec![1]],
            _ => {}
        }

        let mut triangle = vec![vec![1]];

        for _ in 1..num_rows {
            let mut row = vec![1];
            let mut iter = triangle.last().unwrap().iter().peekable();
            loop {
                match (iter.next(), iter.peek()) {
                    (Some(current), Some(next)) => row.push(current + *next),
                    _ => break,
                }
            }
            row.push(1);
            triangle.push(row);
        }

        triangle
    }
}
// @lc code=end
