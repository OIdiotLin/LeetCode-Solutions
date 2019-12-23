/**
 * [21] Merge Two Sorted Lists
 *
 * Merge two sorted linked lists and return it as a new list. The new list should be made by splicing together the nodes of the first two lists.
 * 
 * Example:
 * 
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sentinel = Some(Box::new(ListNode { val: 0, next: None }));
        let mut l1 = l1;
        let mut l2 = l2;
        let mut p = &mut sentinel;

        while l1.is_some() || l2.is_some() {
            if l1.is_none() {
                p.as_mut().unwrap().next = l2;
                break;  // no need for comparison
            }
            if l2.is_none() {
                p.as_mut().unwrap().next = l1;
                break;  // no need for comparison
            }

            let next = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let new_head = l1.as_mut().unwrap().next.take();
                let next = l1.take();
                l1 = new_head;
                next
            } else {
                let new_head = l2.as_mut().unwrap().next.take();
                let next = l2.take();
                l2 = new_head;
                next
            };

            p.as_mut().unwrap().next = next;
            p = &mut p.as_mut().unwrap().next;
        }
        sentinel.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_21_case1() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn test_21_case2() {
        assert_eq!(
            Solution::merge_two_lists(to_list(vec![]), to_list(vec![])),
            to_list(vec![])
        );
    }
}