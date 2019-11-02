/*
 * @lc app=leetcode id=110 lang=rust
 *
 * [110] Balanced Binary Tree
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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::balanced(&root)
    }

    fn balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let left_node = &root.borrow().left;
                let right_node = &root.borrow().right;

                let left_depth = Self::depth(left_node);
                let right_depth = Self::depth(right_node);

                (left_depth - right_depth).abs() <= 1
                    && Self::balanced(left_node)
                    && Self::balanced(right_node)
            }
        }
    }

    fn depth(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                std::cmp::max(
                    Self::depth(&node.borrow().left),
                    Self::depth(&node.borrow().right),
                ) + 1
            }
        }
    }
}
// @lc code=end
