/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut head = head;
        let mut ln = head.as_mut().unwrap();

        loop {
            match ln.next.as_mut() {
                None => return head,
                Some(nln) => {
                    if ln.val == nln.val {
                        ln.next = nln.next.take();
                    } else {
                        ln = ln.next.as_mut().unwrap();
                    }
                }
            }
        }
    }
}
// @lc code=end
