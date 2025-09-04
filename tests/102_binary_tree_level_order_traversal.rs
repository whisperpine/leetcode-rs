// 102. Binary Tree Level Order Traversal
// https://leetcode.com/problems/binary-tree-level-order-traversal/description/
// Topics: Trees.
// Difficulty: Medium.

#[test]
fn test_102_binary_tree_level_order_traversal() {}

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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut vd = VecDeque::new();
        if let Some(r) = root {
            vd.push_back(r);
        }
        while !vd.is_empty() {
            let mut row: Vec<i32> = vec![];
            for _ in 0..vd.len() {
                // `vd` is not empty, thus `unwrap()` is fine here.
                let node = vd.pop_front().unwrap();
                row.push(node.borrow().val);
                if let Some(left) = node.borrow_mut().left.take() {
                    vd.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    vd.push_back(right);
                }
            }
            result.push(row);
        }
        result
    }
}
