/**
 * [7] Reverse Integer
 *
 * Given a 32-bit signed integer, reverse digits of an integer.
 * 
 * Example 1:
 * 
 * 
 * Input: 123
 * Output: 321
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: -123
 * Output: -321
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: 120
 * Output: 21
 * 
 * 
 * Note:<br />
 * Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let sign = x >= 0;
        let mut x = if sign { x as i64 } else { -(x as i64) };
        let mut rev_x = 0i64;
        while x > 0 {
            let digit = x % 10;
            x /= 10;
            rev_x *= 10;
            rev_x += digit;
        }
        rev_x *= if sign { 1 } else { -1 };
        if rev_x > std::i32::MAX as i64 || rev_x < std::i32::MIN as i64 { 0 } else { rev_x as i32 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        assert_eq!(Solution::reverse(2147483647), 0);
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(-3210), -123);
        assert_eq!(Solution::reverse(-2147483648), 0);
        assert_eq!(Solution::reverse(0), 0);
    }
}