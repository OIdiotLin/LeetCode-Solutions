/**
 * [9] Palindrome Number
 *
 * Determine whether an integer is a palindrome. An integer is a palindrome when it reads the same backward as forward.
 * 
 * Example 1:
 * 
 * 
 * Input: 121
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: -121
 * Output: false
 * Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 10
 * Output: false
 * Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 * 
 * 
 * Follow up:
 * 
 * Coud you solve it without converting the integer to a string?
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0) { return false; }
        let mut t = x as i64;
        let mut rev = 0i64;
        while t > 0 {
            rev *= 10;
            rev += t % 10;
            t /= 10;
            if rev > std::i32::MAX as i64 { return false; }
        }
        println!("rev = {}; x = {}", rev, x);
        rev as i32 == x
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-212), false);
        assert_eq!(Solution::is_palindrome(0), true);
    }
}