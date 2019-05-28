/**
 * [10] Regular Expression Matching
 *
 * Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
 * 
 * 
 * '.' Matches any single character.
 * '*' Matches zero or more of the preceding element.
 * 
 * 
 * The matching should cover the entire input string (not partial).
 * 
 * Note:
 * 
 * 
 * 	s could be empty and contains only lowercase letters a-z.
 * 	p could be empty and contains only lowercase letters a-z, and characters like . or *.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input:
 * s = "aa"
 * p = "a"
 * Output: false
 * Explanation: "a" does not match the entire string "aa".
 * 
 * 
 * Example 2:
 * 
 * 
 * Input:
 * s = "aa"
 * p = "a*"
 * Output: true
 * Explanation: '*' means zero or more of the precedeng element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
 * 
 * 
 * Example 3:
 * 
 * 
 * Input:
 * s = "ab"
 * p = ".*"
 * Output: true
 * Explanation: ".*" means "zero or more (*) of any character (.)".
 * 
 * 
 * Example 4:
 * 
 * 
 * Input:
 * s = "aab"
 * p = "c*a*b"
 * Output: true
 * Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".
 * 
 * 
 * Example 5:
 * 
 * 
 * Input:
 * s = "mississippi"
 * p = "mis*is*p*."
 * Output: false
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let s_len = s.len();
        let p_len = p.len();
        let mut matched = vec![vec![false; p_len + 1]; s_len + 1];
        matched[s_len][p_len] = true;

        for i in (0..s_len + 1).rev() {
            for j in (0..p_len).rev() {
                let cur_matched = i < s_len && (s[i] == p[j] || p[j] == '.');
                matched[i][j] = if j + 1 < p_len && p[j + 1] == '*' {
                    matched[i][j + 2] || cur_matched && matched[i + 1][j]
                } else {
                    cur_matched && matched[i + 1][j + 1]
                }
            }
        }

        matched[0][0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(Solution::is_match("ab".to_owned(), ".*".to_owned()), true);
        assert_eq!(Solution::is_match("aab".to_owned(), "c*a*b".to_owned()), true);
        assert_eq!(Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()), false);
    }
}