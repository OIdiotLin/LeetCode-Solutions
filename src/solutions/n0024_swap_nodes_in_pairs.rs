/**
 * [24] Swap Nodes in Pairs
 *
 * Given a linked list, swap every two adjacent nodes and return its head.
 * 
 * You may not modify the values in the list's nodes, only nodes itself may be changed.
 * 
 *  
 * 
 * Example:
 * 
 * 
 * Given 1->2->3->4, you should return the list as 2->1->4->3.
 * 
 * 
 */
pub struct Solution {}

use super::super::utils::linked_list::{ListNode, to_list};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Some(Box::new(ListNode::new(0)));
        sentinel.as_mut().unwrap().next = head;

        let mut p = sentinel.as_mut();
        while p.is_some() {
            let mut l_node = p.as_mut().unwrap().next.take();
            if l_node.is_none() { break; }
            let mut r_node = l_node.as_mut().unwrap().next.take();
            if r_node.is_none() {
                p.as_mut().unwrap().next = l_node;
                break;
            }

            let next_pair = r_node.as_mut().unwrap().next.take();
            l_node.as_mut().unwrap().next = next_pair;
            r_node.as_mut().unwrap().next = l_node;
            p.as_mut().unwrap().next = r_node;
            p = p.unwrap().next.as_mut().unwrap().next.as_mut();
        }

        sentinel.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_24_case1() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![1, 2, 3, 4])),
            to_list(vec![2, 1, 4, 3])
        );
    }

    #[test]
    fn test_24_case2() {
        assert_eq!(Solution::swap_pairs(to_list(vec![])), to_list(vec![]));
    }

    #[test]
    fn test_24_case3() {
        assert_eq!(
            Solution::swap_pairs(to_list(vec![2, 1, 3])),
            to_list(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_24_case4() {
        assert_eq!(Solution::swap_pairs(to_list(vec![2])), to_list(vec![2]));
    }
}