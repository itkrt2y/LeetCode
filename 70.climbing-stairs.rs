/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = vec![0; n as usize];
        Self::climb_stair(0, n, &mut memo)
    }

    fn climb_stair(i: i32, n: i32, memo: &mut Vec<i32>) -> i32 {
        if i > n {
            return 0;
        }
        if i == n {
            return 1;
        }
        if memo[i as usize] > 0 {
            return memo[i as usize];
        }

        memo[i as usize] = Self::climb_stair(i + 1, n, memo) + Self::climb_stair(i + 2, n, memo);
        memo[i as usize]
    }
}
// @lc code=end
