// 226. Invert Binary Tree
// https://leetcode.com/problems/invert-binary-tree/description/
// Topics: Trees.
// Difficulty: Easy.

#[test]
fn test_226_invert_binary_tree() {}

#[derive(Debug)]
pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// ---------------------------------
// copy to leetcode starts from here
// ---------------------------------

use std::cell::RefCell;
use std::rc::Rc;

trait TreeNodeSwap {
    fn swap(&mut self);
    fn swap_all(&mut self);
}

impl TreeNodeSwap for TreeNode {
    fn swap(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }
    fn swap_all(&mut self) {
        if let Some(node) = self.left.as_ref() {
            node.borrow_mut().swap_all()
        }
        if let Some(node) = self.right.as_ref() {
            node.borrow_mut().swap_all()
        }
        self.swap();
    }
}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.inspect(|node| node.borrow_mut().swap_all())
    }
}
