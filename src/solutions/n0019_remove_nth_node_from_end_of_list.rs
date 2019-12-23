/**
 * [19] Remove Nth Node From End of List
 *
 * Given a linked list, remove the n-th node from the end of list and return its head.
 * 
 * Example:
 * 
 * 
 * Given linked list: 1->2->3->4->5, and n = 2.
 * 
 * After removing the second node from the end, the linked list becomes 1->2->3->5.
 * 
 * 
 * Note:
 * 
 * Given n will always be valid.
 * 
 * Follow up:
 * 
 * Could you do this in one pass?
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut sentinel = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;

        let mut p = sentinel.as_ref();
        while p.is_some() {
            p = p.unwrap().next.as_ref();
            len += 1;
        }

        let mut p = sentinel.as_mut();
        for i in 0..len - n - 1 {
            p = p.unwrap().next.as_mut();
        }
        p.as_mut().unwrap().next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();

        sentinel.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19_case1() {
        assert_eq!(
            Solution::remove_nth_from_end(to_list(vec![1, 2, 3, 4, 5]), 2),
            to_list(vec![1, 2, 3, 5])
        );
    }

    #[test]
    fn test_19_case2() {
        assert_eq!(Solution::remove_nth_from_end(to_list(vec![1]), 1), None);
    }
}