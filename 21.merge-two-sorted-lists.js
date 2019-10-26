/*
 * @lc app=leetcode id=21 lang=javascript
 *
 * [21] Merge Two Sorted Lists
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
function mergeTwoLists(l1, l2) {
  const dummy = new ListNode(0);
  let cur = dummy;

  while (l1 || l2) {
    if (l1 === null) {
      cur.next = l2;
      break;
    }
    if (l2 === null) {
      cur.next = l1;
      break;
    }

    if (l1.val < l2.val) {
      cur = cur.next = l1;
      l1 = l1.next;
    } else {
      cur = cur.next = l2;
      l2 = l2.next;
    }
  }

  return dummy.next;
}
// @lc code=end
