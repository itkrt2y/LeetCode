/*
 * @lc app=leetcode id=107 lang=rust
 *
 * [107] Binary Tree Level Order Traversal II
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
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut list: Vec<Vec<i32>> = vec![];
        Self::traverse(&mut list, &root, 0);
        list
    }

    fn traverse(list: &mut Vec<Vec<i32>>, tree_node: &Option<Rc<RefCell<TreeNode>>>, depth: usize) {
        match tree_node {
            None => return,
            Some(node) => {
                if depth >= list.len() {
                    list.insert(0, vec![]);
                }

                let node = node.borrow();
                Self::traverse(list, &node.left, depth + 1);
                Self::traverse(list, &node.right, depth + 1);

                let index = list.len() - depth - 1;
                match list.get_mut(index) {
                    None => {}
                    Some(list_item) => list_item.push(node.val),
                }
            }
        }
    }
}
// @lc code=end
