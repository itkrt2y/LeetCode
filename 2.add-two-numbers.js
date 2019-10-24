/*
 * @lc app=leetcode id=2 lang=javascript
 *
 * [2] Add Two Numbers
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * function ListNode(val) {
 *     this.val = val;
 *     this.next = null;
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
function addTwoNumbers(l1, l2, c = null) {
  let val = 0;

  let l1next = null;
  if (l1) {
    val += l1.val;
    l1next = l1.next;
  }

  let l2next = null;
  if (l2) {
    val += l2.val;
    l2next = l2.next;
  }

  if (c && c.val >= 10) {
    val += 1;
    c.val -= 10;
  }

  const l = new ListNode(val);

  if (c) {
    c.next = l;
  }

  if (l1next || l2next || l.val >= 10) {
    addTwoNumbers(l1next, l2next, l);
  }

  return l;
}
// @lc code=end
