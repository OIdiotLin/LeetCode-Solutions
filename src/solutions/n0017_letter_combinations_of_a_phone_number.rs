/**
 * [17] Letter Combinations of a Phone Number
 *
 * Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent.
 * 
 * A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
 * 
 * <img src="http://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png" />
 * 
 * Example:
 * 
 * 
 * Input: "23"
 * Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
 * 
 * 
 * Note:
 * 
 * Although the above answer is in lexicographical order, your answer could be in any order you want.
 * 
 */

use std::collections::HashMap;

pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if digits.len() == 0 {
            return res;
        }
        let mut char_map: HashMap<char, String> = HashMap::new();
        char_map.insert('2', String::from("abc"));
        char_map.insert('3', String::from("def"));
        char_map.insert('4', String::from("ghi"));
        char_map.insert('5', String::from("jkl"));
        char_map.insert('6', String::from("mno"));
        char_map.insert('7', String::from("pqrs"));
        char_map.insert('8', String::from("tuv"));
        char_map.insert('9', String::from("wxyz"));

        Solution::dfs(&digits, &char_map, String::new(), &mut res);

        res
    }

    fn dfs(digits: &String, char_map: &HashMap<char, String>, pre: String, res: &mut Vec<String>) {
        if pre.len() == digits.len() {
            res.push(pre);
            return;
        }
        let next_chars = char_map.get(&digits.chars().nth(pre.len()).unwrap()).unwrap().chars();
        for ch in next_chars {
            let mut new_s = pre.clone();
            new_s.push(ch);
            Solution::dfs(digits, char_map, new_s, res);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_17() {
        let mut case1 = ["cf", "af", "bf", "cd", "ce", "ad", "ae", "bd", "be"];
        case1.sort();
        assert_eq!(Solution::letter_combinations(String::from("23")), case1);
    }
}