/**
 * [5] Longest Palindromic Substring
 *
 * Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
 * 
 * Example 1:
 * 
 * 
 * Input: "babad"
 * Output: "bab"
 * Note: "aba" is also a valid answer.
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "cbbd"
 * Output: "bb"
 * 
 * 
 */
pub struct Solution {}

// submission codes start here
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut new_s = vec!['@'];
        new_s.append(&mut(s.split("").collect::<Vec<&str>>().join("#").chars().collect()));
        new_s.push('$');
        let len = new_s.len();
        println!("{:?}", new_s);

        let mut max_len = 0usize;
        let mut max_center = 0usize;
        let mut rad = vec![0usize; len];
        let mut center = 1usize;
        let mut right = 0usize;

        for i in 1..len-1 {
            let mirror_i = center * 2 - i;
            if right > i {
                rad[i] = std::cmp::min(right - i, rad[mirror_i]);
            }
            while new_s[i + rad[i] + 1] == new_s[i - rad[i] - 1] {
                rad[i] += 1;
                println!("{}, {}", i, rad[i]);
            }
            if i + rad[i] > right {
                center = i;
                right = rad[i] + i;
            }
            if rad[i] > max_len {
                max_len = rad[i];
                max_center = i;
            }
        }
        let start = max_center - max_len;
        let end = max_center + max_len;
        (&new_s[start..end]).iter().filter(|x| **x != '#').collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        assert_eq!(Solution::longest_palindrome("fcabaceg".to_owned()), "cabac");
        assert_eq!(Solution::longest_palindrome("aaaaa".to_owned()), "aaaaa");
        assert_eq!(Solution::longest_palindrome("babab".to_owned()), "babab");
        assert_eq!(Solution::longest_palindrome("babcd".to_owned()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), "bb");
        assert_eq!(Solution::longest_palindrome("".to_owned()), "");
    }
}