/**
 * [124] Binary Tree Maximum Path Sum
 *
 * Given a non-empty binary tree, find the maximum path sum.
 * 
 * For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections. The path must contain at least one node and does not need to go through the root.
 * 
 * Example 1:
 * 
 * 
 * Input: [1,2,3]
 * 
 *        1
 *       / \
 *      2   3
 * 
 * Output: 6
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: [-10,9,20,null,null,15,7]
 * 
 *    -10
 *    / \
 *   9  20
 *     /  \
 *    15   7
 * 
 * Output: 42
 * 
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
use std::cmp::max;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cur_max_sum = i32::min_value();
        Solution::dp(root.as_ref(), &mut cur_max_sum);
        cur_max_sum
    }

    fn dp(root: Option<&Rc<RefCell<TreeNode>>>, cur_max_sum: &mut i32) -> i32 {
        if let Some(c) = root {
            let sum_l = Solution::dp(c.borrow().left.as_ref(), cur_max_sum);
            let sum_r = Solution::dp(c.borrow().right.as_ref(), cur_max_sum);
            *cur_max_sum = max(
                *cur_max_sum,
                c.borrow().val + max(0, sum_l) + max(0, sum_r),
            );
            c.borrow().val + max(0, max(sum_l, sum_r))
        } else {
            0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_124_case0() {
        assert_eq!(Solution::max_path_sum(tree![1,2,3]), 6);
    }

    #[test]
    fn test_124_case1() {
        assert_eq!(Solution::max_path_sum(tree![-10,9,20,null,null,15,7]), 42);
    }
}