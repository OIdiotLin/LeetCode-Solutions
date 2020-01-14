/**
 * [112] Path Sum
 *
 * Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.
 * 
 * Note: A leaf is a node with no children.
 * 
 * Example:
 * 
 * Given the below binary tree and sum = 22,
 * 
 * 
 *       5
 *      / \
 *     4   8
 *    /   / \
 *   11  13  4
 *  /  \      \
 * 7    2      1
 * 
 * 
 * return true, as there exist a root-to-leaf path 5->4->11->2 which sum is 22.
 * 
 */
pub struct Solution {}

use super::super::utils::tree::{TreeNode, to_tree};

// submission codes start here

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
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if root.is_none() { false } else {
            Solution::find(root.as_ref(), sum)
        }
    }
    fn find(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        if let Some(c) = root {
            let rest = sum - c.borrow().val;
            if c.borrow().left.is_none() && c.borrow().right.is_none() {
                rest == 0
            } else {
                Solution::find(c.borrow().left.as_ref(), rest) ||
                    Solution::find(c.borrow().right.as_ref(), rest)
            }
        } else {
            false
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_112_case0() {
        assert_eq!(Solution::has_path_sum(tree![1], 1), true);
    }

    #[test]
    fn test_112_case1() {
        assert_eq!(Solution::has_path_sum(tree![1], 2), false);
    }

    #[test]
    fn test_112_case2() {
        assert_eq!(Solution::has_path_sum(
            tree![5,4,8,11,null,13,4,7,2,null,null,null,1], 22,
        ), true);
    }

    #[test]
    fn test_112_case3() {
        assert_eq!(Solution::has_path_sum(tree![], 0), false);
    }
}