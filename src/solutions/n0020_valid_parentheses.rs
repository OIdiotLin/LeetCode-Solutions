/**
 * [20] Valid Parentheses
 *
 * Given a string containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
 * 
 * An input string is valid if:
 * 
 * <ol>
 * 	Open brackets must be closed by the same type of brackets.
 * 	Open brackets must be closed in the correct order.
 * </ol>
 * 
 * Note that an empty string is also considered valid.
 * 
 * Example 1:
 * 
 * 
 * Input: "()"
 * Output: true
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: "()[]{}"
 * Output: true
 * 
 * 
 * Example 3:
 * 
 * 
 * Input: "(]"
 * Output: false
 * 
 * 
 * Example 4:
 * 
 * 
 * Input: "([)]"
 * Output: false
 * 
 * 
 * Example 5:
 * 
 * 
 * Input: "{[]}"
 * Output: true
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            if Solution::check(&mut stack, ch) == false {
                return false;
            }
        }
        stack.is_empty()
    }

    fn check(stack: &mut Vec<char>, ch: char) -> bool {
        match stack.pop() {
            Some(top) =>
                match ch {
                    ')' => top == '(',
                    ']' => top == '[',
                    '}' => top == '{',
                    '(' | '[' | '{' => {
                        stack.push(top);
                        stack.push(ch);
                        true
                    }
                    _ => false
                }
            None => {
                stack.push(ch);
                true
            }
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_20_case1() {
        assert_eq!(Solution::is_valid("{{}{}()[()]}".to_string()), true);
    }

    #[test]
    fn test_20_case2() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }

    #[test]
    fn test_20_case3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_20_case4() {
        assert_eq!(Solution::is_valid("[".to_string()), false);
    }
}