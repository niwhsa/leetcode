// 94. Binary Tree Inorder Traversal (Easy)
// https://leetcode.com/problems/binary-tree-inorder-traversal/
//
// Time: O(n), Space: O(h) where h = tree height

use std::cell::RefCell;
use std::rc::Rc;

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

pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = root;

        while current.is_some() || !stack.is_empty() {
            // Traverse left subtree, pushing nodes onto stack
            while let Some(node) = current {
                stack.push(Rc::clone(&node));
                current = node.borrow().left.clone();
            }

            // Visit node (pop from stack)
            if let Some(node) = stack.pop() {
                result.push(node.borrow().val);
                // Move to right subtree
                current = node.borrow().right.clone();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn test_example() {
        // [1,null,2,3] → [1,3,2]
        let tree = node(1, None, node(2, node(3, None, None), None));
        assert_eq!(Solution::inorder_traversal(tree), vec![1, 3, 2]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::inorder_traversal(None), vec![]);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::inorder_traversal(node(1, None, None)), vec![1]);
    }

    #[test]
    fn test_balanced() {
        //     2
        //    / \
        //   1   3
        let tree = node(2, node(1, None, None), node(3, None, None));
        assert_eq!(Solution::inorder_traversal(tree), vec![1, 2, 3]);
    }
}

