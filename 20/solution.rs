struct Solution {}
impl Solution {
    pub fn token(ch: char) -> u8 {
        match ch {
            '[' | ']' => 1,
            '(' | ')' => 2,
            '{' | '}' => 3,
            _ => 4
        }
    }
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '[' | '(' | '{' => stack.push(Solution::token(ch)),
                ']' | ')' | '}' => {
                    if let Some(t) = stack.pop() {
                        if t != Solution::token(ch) {
                            return false;
                        }
                        continue;
                    }
                    return false;
                },
                _ => {}
            }
        }
        stack.len() == 0
    }
}


fn main () {
    let s1 = "{}{}{}{}{}(){{([])}}".to_string();
    let s2 = "{}{()}{)}{}{}(){{([])}}".to_string();
    let s3 = "]".to_string();
    println!("s1:{}", Solution::is_valid(s1));
    println!("s2:{}", Solution::is_valid(s2));
    println!("s3:{}", Solution::is_valid(s3));
}
