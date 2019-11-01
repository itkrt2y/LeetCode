/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
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
use std::cmp::max;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::depth(&root, 0)
    }

    fn depth(tree_node: &Option<Rc<RefCell<TreeNode>>>, mut current_depth: i32) -> i32 {
        match tree_node {
            None => current_depth,
            Some(node) => {
                current_depth += 1;
                let node = node.borrow();
                max(
                    Self::depth(&node.left, current_depth),
                    Self::depth(&node.right, current_depth),
                )
            }
        }
    }
}
// @lc code=end
