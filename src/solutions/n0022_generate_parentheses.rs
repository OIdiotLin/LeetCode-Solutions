use std::collections::VecDeque;

/**
 * [22] Generate Parentheses
 *
 * 
 * Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
 * 
 * 
 * 
 * For example, given n = 3, a solution set is:
 * 
 * 
 * [
 *   "((()))",
 *   "(()())",
 *   "(())()",
 *   "()(())",
 *   "()()()"
 * ]
 * 
 */
pub struct Solution {}

// submission codes start here

struct SearchTreeNode {
    cur_string: String,
    l_count: i32,
    r_count: i32,
}

impl SearchTreeNode {
    #[inline]
    pub fn new(cur_string: String, l_count: i32, r_count: i32) -> Self {
        SearchTreeNode { cur_string, l_count, r_count }
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];

        let mut queue: VecDeque<SearchTreeNode> = VecDeque::new();
        queue.push_back(SearchTreeNode::new("".to_string(), 0, 0));

        for i in 0..n << 1 {
            for j in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                if node.l_count < n {
                    let mut new_string = node.cur_string.clone();
                    new_string.push('(');
                    queue.push_back(SearchTreeNode::new(
                        new_string, node.l_count + 1, node.r_count,
                    ));
                }
                if node.r_count < n && node.l_count > node.r_count {
                    let mut new_string = node.cur_string.clone();
                    new_string.push(')');
                    queue.push_back(SearchTreeNode::new(
                        new_string, node.l_count, node.r_count + 1,
                    ));
                }
            }
        }

        while let Some(node) = queue.pop_front() {
            res.push(node.cur_string.clone());
        }

        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22_case1() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
    }

    #[test]
    fn test_22_case2() {
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
    }

    #[test]
    fn test_22_case3() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}