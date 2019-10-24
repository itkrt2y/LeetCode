/*
 * @lc app=leetcode id=9 lang=javascript
 *
 * [9] Palindrome Number
 */

// @lc code=start
/**
 * @param {number} x
 * @return {boolean}
 */
function isPalindrome(x) {
  if (x < 0) {
    return false;
  }

  if (0 < x && x < 10) {
    return true;
  }

  const origX = x;
  let rev = 0;
  while (x !== 0) {
    rev = rev * 10 + (x % 10);
    x = Math.floor(x / 10);
  }

  return origX === rev;
}
// @lc code=end
