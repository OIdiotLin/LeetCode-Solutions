/**
 * [113] Path Sum II
 *
 * Given a binary tree and a sum, find all root-to-leaf paths where each path's sum equals the given sum.
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
 *  /  \    / \
 * 7    2  5   1
 * 
 * 
 * Return:
 * 
 * 
 * [
 *    [5,4,11,2],
 *    [5,8,4,5]
 * ]
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

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        if root.is_some() {
            Solution::find(root.as_ref(), sum, vec![], &mut paths);
        }
        paths
    }

    fn find(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32,
            mut cur_path: Vec<i32>, paths: &mut Vec<Vec<i32>>) {
        if let Some(c) = root {
            let rest = sum - c.borrow().val;
            if c.borrow().left.is_none() && c.borrow().right.is_none() {
                if rest == 0 {
                    cur_path.push(c.borrow().val);
                    paths.push(cur_path);
                }
            } else {
                cur_path.push(c.borrow().val);
                Solution::find(c.borrow().left.as_ref(), rest,
                               cur_path.clone(), paths);
                Solution::find(c.borrow().right.as_ref(), rest,
                               cur_path.clone(), paths);
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_113_case0() {
        assert_eq!(
            Solution::path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,5,1], 22),
            vec![
                vec![5, 4, 11, 2],
                vec![5, 8, 4, 5]
            ]
        );
    }

    #[test]
    fn test_113_case1() {
        assert_eq!(
            Solution::path_sum(tree![0], 22),
            vec![] as Vec<Vec<i32>>
        );
    }
}