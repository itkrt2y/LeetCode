/*
 * @lc app=leetcode id=122 lang=rust
 *
 * [122] Best Time to Buy and Sell Stock II
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i - 1] < prices[i] {
                profit += prices[i] - prices[i - 1];
            }
        }

        profit
    }
}
// @lc code=end
