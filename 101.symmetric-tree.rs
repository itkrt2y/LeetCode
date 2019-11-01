/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_mirror(&root, &root)
    }

    fn is_mirror(
        tree1: &Option<Rc<RefCell<TreeNode>>>,
        tree2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (tree1, tree2) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(tree1), Some(tree2)) => {
                let tree1 = tree1.borrow();
                let tree2 = tree2.borrow();

                tree1.val == tree2.val
                    && Self::is_mirror(&tree1.left, &tree2.right)
                    && Self::is_mirror(&tree1.right, &tree2.left)
            }
        }
    }
}
// @lc code=end
