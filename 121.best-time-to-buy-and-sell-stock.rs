/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::max_value();
        let mut max = 0;

        for i in 0..prices.len() {
            if min > prices[i] {
                min = prices[i];
            } else if prices[i] - min > max {
                max = prices[i] - min;
            }
        }

        max
    }
}
// @lc code=end
