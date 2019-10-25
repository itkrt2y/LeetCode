/*
 * @lc app=leetcode id=13 lang=javascript
 *
 * [13] Roman to Integer
 */

// @lc code=start
/**
 * @param {string} s
 * @return {number}
 */
function romanToInt(s) {
  const chars = s.split("");

  let num = 0;
  let current;
  let peek;

  for (let i = 0; i < chars.length; i++) {
    current = chars[i];
    peek = chars[i + 1];

    switch (current) {
      case "I":
        switch (peek) {
          case "V":
            num += 4;
            i++;
            break;
          case "X":
            num += 9;
            i++;
            break;
          default:
            num += 1;
            break;
        }

        break;
      case "X":
        switch (peek) {
          case "L":
            num += 40;
            i++;
            break;
          case "C":
            num += 90;
            i++;
            break;
          default:
            num += 10;
            break;
        }

        break;
      case "C":
        switch (peek) {
          case "D":
            num += 400;
            i++;
            break;
          case "M":
            num += 900;
            i++;
            break;
          default:
            num += 100;
            break;
        }

        break;

      case "V":
        num += 5;
        break;
      case "L":
        num += 50;
        break;
      case "D":
        num += 500;
        break;
      case "M":
        num += 1000;
        break;
    }
  }

  return num;
}
// @lc code=end
