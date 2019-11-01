/*
 * @lc app=leetcode id=100 lang=rust
 *
 * [100] Same Tree
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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::is_same(&p, &q)
    }

    fn is_same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(pp), Some(qq)) => {
                let pp = pp.borrow();
                let qq = qq.borrow();

                if pp.val != qq.val {
                    return false;
                }

                if !Self::is_same(&pp.left, &qq.left) {
                    return false;
                }

                if !Self::is_same(&pp.right, &qq.right) {
                    return false;
                }

                true
            }
        }
    }
}
// @lc code=end
