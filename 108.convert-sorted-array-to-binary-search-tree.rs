/*
 * @lc app=leetcode id=108 lang=rust
 *
 * [108] Convert Sorted Array to Binary Search Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let end: i32 = nums.len() as i32 - 1;
        Self::create_node(&nums, 0, end)
    }

    fn create_node(nums: &Vec<i32>, start: i32, end: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if start > end {
            return None;
        }

        let mid = ((start + end) as f32 / 2.0).ceil() as i32;
        let node = Rc::new(RefCell::new(TreeNode::new(nums[mid as usize])));
        node.borrow_mut().left = Self::create_node(nums, start, mid - 1);
        node.borrow_mut().right = Self::create_node(nums, mid + 1, end);
        Some(node)
    }
}
// @lc code=end
