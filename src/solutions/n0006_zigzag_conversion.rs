/**
 * [6] ZigZag Conversion
 *
 * The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
 * 
 * 
 * P   A   H   N
 * A P L S I I G
 * Y   I   R
 * 
 * 
 * And then read line by line: "PAHNAPLSIIGYIR"
 * 
 * Write the code that will take a string and make this conversion given a number of rows:
 * 
 * 
 * string convert(string s, int numRows);
 * 
 * Example 1:
 * 
 * 
 * Input: s = "PAYPALISHIRING", numRows = 3
 * Output: "PAHNAPLSIIGYIR"
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: s = "PAYPALISHIRING", numRows = 4
 * Output: "PINALSIGYAHRPI"
 * Explanation:
 * 
 * P     I    N
 * A   L S  I G
 * Y A   H R
 * P     I
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;

        if num_rows == 1 {
            return s;
        }

        let mut rows = vec![String::new(); num_rows];
        let group_size = 2 * num_rows - 2;
        for (idx, ch) in s.chars().enumerate() {
            let pos_in_group = idx % group_size;
            if pos_in_group < num_rows {
                rows[pos_in_group].push(ch);
            } else {
                rows[2 * num_rows - 2 - pos_in_group].push(ch);
            }
        }
        rows.iter().fold(String::new(), |mut s, x| {
            s.push_str(x.as_str());
            s
        })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        assert_eq!(Solution::convert("LEETCODEISHIRING".to_owned(), 3), "LCIRETOESIIGEDHN".to_owned());
        assert_eq!(Solution::convert("LEETCODEISHIRING".to_owned(), 4), "LDREOEIIECIHNTSG".to_owned());
        assert_eq!(Solution::convert("AAAAA".to_owned(), 1), "AAAAA".to_owned());
    }
}