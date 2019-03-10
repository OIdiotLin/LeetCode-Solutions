struct Solution {}
impl Solution {
    pub fn sub_str(s: &String, sub_len: usize) -> Vec<String> {
        s.chars().collect::<Vec<char>>()
            .chunks(sub_len as usize)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<String>>()
    }

    pub fn reverse_substr(s: &String, k: usize) -> String {
        println!("reverse_substr({})", s);
        if s.len() >= k && s.len() <= 2*k {
            let mut res = String::new();
            let subs = Solution::sub_str(&s, k);
            let s1: &String = &subs[0].chars().rev().collect::<String>();
            res.push_str(&s1);
            if subs.len() == 2 as usize {
                res.push_str(&subs[1]);
            }
            return res;
        } else {
            return s.chars().rev().collect::<String>();
        }
    }

    pub fn reverse_str(s: String, k: i32) -> String {
        let subs = Solution::sub_str(&s, 2*k as usize);
        let mut res = String::new();
        for sub in subs {
            res.push_str(&Solution::reverse_substr(&sub, k as usize));
        }
        res
    }
}

use std::io;
use std::str::FromStr;

fn main() {
    // let s = String::from("abcdefg");
    let mut s = String::new();
    let mut k = String::new();
    io::stdin().read_line(&mut s).expect("error");
    io::stdin().read_line(&mut k).expect("error");
    println!("{}", Solution::reverse_str(s.trim().to_string(), FromStr::from_str(&k.trim()).unwrap()));
}