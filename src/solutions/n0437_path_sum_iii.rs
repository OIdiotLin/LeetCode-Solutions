use std::cell::RefCell;
use std::collections::HashMap;
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

use super::super::utils::tree::{to_tree, TreeNode};

/**
 * [437] Path Sum III
 *
 * You are given a binary tree in which each node contains an integer value.
 * 
 * Find the number of paths that sum to a given value.
 * 
 * The path does not need to start or end at the root or a leaf, but it must go downwards
 * (traveling only from parent nodes to child nodes).
 * 
 * The tree has no more than 1,000 nodes and the values are in the range -1,000,000 to 1,000,000.
 * 
 * Example:
 * 
 * root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8
 * 
 *       10
 *      /  \
 *     5   -3
 *    / \    \
 *   3   2   11
 *  / \   \
 * 3  -2   1
 * 
 * Return 3. The paths that sum to 8 are:
 * 
 * 1.  5 -> 3
 * 2.  5 -> 2 -> 1
 * 3. -3 -> 11
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

//impl Solution {
//    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
//        let target = sum;
//        let mut res = 0;
//        let mut counter: HashMap<i32, i32> = HashMap::new();
//        counter.insert(0,1);
//        Solution::pre_order(root.as_ref(), 0, target, &mut counter, &mut res);
//        res
//    }
//
//    fn pre_order(root: Option<&Rc<RefCell<TreeNode>>>, sum: i32, target: i32,
//                 sum_counter: &mut HashMap<i32, i32>, res: &mut i32) {
//        if let Some(c) = root {
//            println!("{:?}", c.borrow().val);
//            let cur_sum = sum + c.borrow().val;
//            let rest = cur_sum - target;
//            let rest_path_count = *sum_counter.get(&rest).unwrap_or(&0);
//            *res += rest_path_count;
//            *sum_counter.entry(cur_sum).or_insert(0) += 1;
//            println!("set = {:?}", sum_counter);
//
//            Solution::pre_order(c.borrow().left.as_ref(), cur_sum, target, sum_counter, res);
//            Solution::pre_order(c.borrow().right.as_ref(), cur_sum, target, sum_counter, res);
//            *sum_counter.get_mut(&cur_sum).unwrap() -= 1;
//        }
//    }
//}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> i32 {
        Solution::path_sum_recurse(&root, vec![], sum) as i32
    }

    pub fn path_sum_recurse(root: &Option<Rc<RefCell<TreeNode>>>, mut values: Vec<i32>, sum: i32) -> usize {
        values.push(0);
        match root {
            None => 0,
            Some(node_ref) => {
                let node_ref_borrow = node_ref.borrow();
                let mut count = 0usize;
                for v in &mut values {
                    *v += node_ref_borrow.val;
                    if *v == sum {
                        count += 1;
                    }
                }
                count
                    + Solution::path_sum_recurse(&node_ref_borrow.left, values.clone(), sum)
                    + Solution::path_sum_recurse(&node_ref_borrow.right, values, sum)
            }
        }
    }
}
// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_437_case0() {
        assert_eq!(Solution::path_sum(tree![10,5,-3,3,2,null,11,3,-2,null,1], 8), 3);
    }

    #[test]
    fn test_437_case1() {
        assert_eq!(Solution::path_sum(tree![10,5,-3,3,2,null,11,3,-2,null,1], 1), 2);
    }

    #[test]
    fn test_437_case2() {
        assert_eq!(Solution::path_sum(tree![], 1), 0);
    }

    #[test]
    fn test_437_case3() {
        assert_eq!(Solution::path_sum(tree![5,4,8,11,null,13,4,7,2,null,null,5,1], 22), 3);
    }
}