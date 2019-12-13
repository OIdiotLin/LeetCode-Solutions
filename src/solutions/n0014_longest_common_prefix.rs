use std::cmp::min;

/**
 * [14] Longest Common Prefix
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 * 
 * If there is no common prefix, return an empty string "".
 * 
 * Example 1:
 * 
 * 
 * Input: ["flower","flow","flight"]
 * Output: "fl"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 * 
 * 
 * Note:
 * 
 * All given inputs are in lowercase letters a-z.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = "".to_string();

        if strs.is_empty() {
            return res;
        }

        let min_len = strs.iter().fold(0xFFFF, |min_len, str| min(min_len, str.len()));
        let mut base_s = strs.iter().nth(0).unwrap().chars();
        for i in 0..min_len {
            let base_c = base_s.nth(0).unwrap();
            let mut flag = true;
            for string in &strs {
                if string.chars().nth(i).unwrap() != base_c {
                    flag = false;
                    break;
                }
            }
            if flag {
                res.push(base_c);
            } else {
                break;
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_14() {
        assert_eq!(Solution::longest_common_prefix(
            vec!["dog", "racecar", "car"].iter().map(|s| s.to_string()).collect()
        ), "");
        assert_eq!(Solution::longest_common_prefix(
            vec!["flower", "flow", "flight"].iter().map(|s| s.to_string()).collect()
        ), "fl");
        assert_eq!(Solution::longest_common_prefix(
            vec!["aba", "cba"].iter().map(|s| s.to_string()).collect()
        ), "");
    }
}