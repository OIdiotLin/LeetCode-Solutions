/**
 * [23] Merge k Sorted Lists
 *
 * Merge k sorted linked lists and return it as one sorted list. Analyze and describe its complexity.
 * 
 * Example:
 * 
 * 
 * Input:
 * [
 *   1->4->5,
 *   1->3->4,
 *   2->6
 * ]
 * Output: 1->1->2->3->4->4->5->6
 * 
 * 
 */
pub struct Solution {}

use super::super::utils::linked_list::{ListNode, to_list};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

struct HeapNode {
    val: i32,
    idx: usize,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.val.cmp(&other.val).reverse())
    }
}

impl Eq for HeapNode {}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.val.eq(&other.val)
    }
}


impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<HeapNode> = BinaryHeap::new();
        for (idx, head) in lists.iter().enumerate() {
            head.as_ref().and_then(|p| Some(heap.push(HeapNode { val: p.val, idx })));
        }
        Solution::build(lists, &mut heap)
    }

    fn build(mut lists: Vec<Option<Box<ListNode>>>, heap: &mut BinaryHeap<HeapNode>) -> Option<Box<ListNode>> {
        heap.pop().map(|head| {
            let next = lists[head.idx].take().unwrap().next;
            next.as_ref().and_then(|p| Some(heap.push(HeapNode { val: p.val, idx: head.idx })));
            lists[head.idx] = next;
            Box::new(ListNode {
                val: head.val,
                next: Solution::build(lists, heap),
            })
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_23_case1() {
        assert_eq!(
            Solution::merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6]),
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        );
    }

    #[test]
    fn test_23_case2() {
        assert_eq!(Solution::merge_k_lists(vec![]), None);
    }
}