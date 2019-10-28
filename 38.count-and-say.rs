/*
 * @lc app=leetcode id=38 lang=rust
 *
 * [38] Count and Say
 */

// @lc code=start
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut nums = vec![1];

        for _ in 1..n {
            nums = {
                let mut ret = vec![];
                let mut cur = nums[0];
                let mut repeat = 1;

                for c in nums[1..].iter() {
                    if *c == cur {
                        repeat += 1;
                    } else {
                        ret.push(repeat);
                        ret.push(cur);
                        cur = *c;
                        repeat = 1;
                    }
                }

                ret.push(repeat);
                ret.push(cur);

                ret
            }
        }

        nums.iter().map(|x| x.to_string()).collect::<String>()
    }
}
// @lc code=end
