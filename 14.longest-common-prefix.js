/*
 * @lc app=leetcode id=14 lang=javascript
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
/**
 * @param {string[]} strs
 * @return {string}
 */
function longestCommonPrefix(strs) {
  if (strs.length === 0) {
    return "";
  }

  const firstStr = strs[0];
  if (firstStr.length === 0) {
    return "";
  }

  let prefix;
  if (matchChar(strs, 0, firstStr[0])) {
    prefix = firstStr[0];
  } else {
    return "";
  }

  for (let i = 1; i < firstStr.length; i++) {
    const char = firstStr[i];
    if (matchChar(strs, i, char)) {
      prefix += char;
    }
  }

  return prefix;
}

function matchChar(strs, index, char) {
  for (let i = 1; i < strs.length; i++) {
    if (char !== strs[i][index]) {
      return false;
    }
  }

  return true;
}
// @lc code=end
