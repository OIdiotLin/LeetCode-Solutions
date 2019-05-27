/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string, find the length of the longest substring without repeating characters.
 * 
 * <div>
 * Example 1:
 * 
 * 
 * Input: <span id="example-input-1-1">"abcabcbb"</span>
 * Output: <span id="example-output-1">3 
 * Explanation:</span> The answer is "abc", with the length of 3. 
 * 
 * 
 * <div>
 * Example 2:
 * 
 * 
 * Input: <span id="example-input-2-1">"bbbbb"</span>
 * Output: <span id="example-output-2">1
 * </span><span id="example-output-1">Explanation: T</span>he answer is "b", with the length of 1.
 * 
 * 
 * <div>
 * Example 3:
 * 
 * 
 * Input: <span id="example-input-3-1">"pwwkew"</span>
 * Output: <span id="example-output-3">3
 * </span><span id="example-output-1">Explanation: </span>The answer is "wke", with the length of 3. 
 *              Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
 * 
 * </div>
 * </div>
 * </div>
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
        pub fn length_of_longest_substring(s: String) -> i32 {
            let window: Vec<char> = s.chars().collect();
            let len = s.len();
            let mut ans = 0usize;

            let mut l = 0;
            let mut r = 0;
            while r < len {
                for i in l..r {
                    if window[r] == window[i] {
                        l = i + 1;
                        break;
                    }
                }
                ans = std::cmp::max(ans, r - l + 1);
                r += 1;
            }

            ans as i32
        }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("wlmmznb".to_string()), 4);
    }
}