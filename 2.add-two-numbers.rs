/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::add_list_nodes(l1, l2, 0)
    }

    fn add_list_nodes(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2, carry) {
            (None, None, 0) => None,
            (None, None, c) => Some(Box::new(ListNode::new(c))),
            (Some(l), None, _) | (None, Some(l), _) => {
                let sum = l.val + carry;
                Some(Box::new(ListNode {
                    next: Self::add_list_nodes(l.next, None, sum / 10),
                    val: sum % 10,
                }))
            }
            (Some(l1), Some(l2), _) => {
                let sum = l1.val + l2.val + carry;
                Some(Box::new(ListNode {
                    next: Self::add_list_nodes(l1.next, l2.next, sum / 10),
                    val: sum % 10,
                }))
            }
        }
    }
}
// @lc code=end
