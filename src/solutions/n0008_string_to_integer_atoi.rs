/**
 * [8] String to Integer (atoi)
 *
 * Implement <span>atoi</span> which converts a string to an integer.
 * 
 * The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.
 * 
 * The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.
 * 
 * If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.
 * 
 * If no valid conversion could be performed, a zero value is returned.
 * 
 * Note:
 * 
 * 
 * 	Only the space character ' ' is considered as whitespace character.
 * 	Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. If the numerical value is out of the range of representable values, INT_MAX (2^31 - 1) or INT_MIN (-2^31) is returned.
 * 
 * 
 * Example 1:
 * 
 * 
 * Input: "42"
 * Output: 42
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "   -42"
 * Output: -42
 * Explanation: The first non-whitespace character is '-', which is the minus sign.
 *              Then take as many numerical digits as possible, which gets 42.
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "4193 with words"
 * Output: 4193
 * Explanation: Conversion stops at digit '3' as the next character is not a numerical digit.
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: "words and 987"
 * Output: 0
 * Explanation: The first non-whitespace character is 'w', which is not a numerical 
 *              digit or a +/- sign. Therefore no valid conversion could be performed.
 * 
 * Example 5:
 * 
 * 
 * Input: "-91283472332"
 * Output: -2147483648
 * Explanation: The number "-91283472332" is out of the range of a 32-bit signed integer.
 *              Thefore INT_MIN (-2^31) is returned.
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut neg = false;
        let mut res = 0i64;
        let mut num_flag = false;
        for ch in str.chars() {
            if num_flag {
                match ch {
                    '0'...'9' => {
                        res *= 10;
                        res += ch.to_digit(10).unwrap() as i64;
                        if res > std::i32::MAX as i64 { break }
                    }
                    _ => break
                }
            } else {
                match ch {
                    '0'...'9' => {
                        num_flag = true;
                        res *= 10;
                        res += ch.to_digit(10).unwrap() as i64;
                    }
                    ' ' => {}
                    '+' => { num_flag = true; }
                    '-' => {
                        num_flag = true;
                        neg = true;
                    }
                    _ => return 0
                }
            }
        }
        res = if neg { -res } else { res };
        res = std::cmp::min(res, std::i32::MAX as i64);
        res = std::cmp::max(res, std::i32::MIN as i64);
        res as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_8() {
        assert_eq!(Solution::my_atoi("9223372036854775808".to_owned()), std::i32::MAX);
        assert_eq!(Solution::my_atoi("-9223372036854775808".to_owned()), std::i32::MIN);
        assert_eq!(Solution::my_atoi("4193 with words ".to_owned()), 4193);
        assert_eq!(Solution::my_atoi("    -42 ".to_owned()), -42);
        assert_eq!(Solution::my_atoi("1231241".to_owned()), 1231241);
        assert_eq!(Solution::my_atoi("2147483648".to_owned()), std::i32::MAX);
        assert_eq!(Solution::my_atoi("-2147483649".to_owned()), std::i32::MIN);
        assert_eq!(Solution::my_atoi("asf ns ie".to_owned()), 0);
        assert_eq!(Solution::my_atoi("with words 4193".to_owned()), 0);
    }
}