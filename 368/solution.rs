struct Solution {}
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_sorted = nums;
        nums_sorted.sort_unstable();

        if nums_sorted.len() == 1 {
            return vec![nums_sorted[0]];
        }
        if nums_sorted.len() == 0 {
            return vec![];
        }

        let mut ans: Vec<i32> = Vec::new();
        let mut f = vec![1; nums_sorted.len()];
        
        for i in 1..nums_sorted.len() {
            for j in (0..i).rev() {
                if nums_sorted[i] % nums_sorted[j] == 0 {
                    f[i] = std::cmp::max(f[i], f[j] + 1);
                }
            }
        }

        let mut f_max = *(f.iter().max().unwrap());
        for i in (0..f.len()).rev() {
            if f_max == f[i] {
                match ans.last() {
                    Some(last) => {
                        if last % nums_sorted[i] == 0 {
                            ans.push(nums_sorted[i]);
                            f_max -= 1;
                        }
                    },
                    None => {
                        ans.push(nums_sorted[i]);
                        f_max -= 1;
                    }
                }
            }
        }

        ans
    }
}


fn main() {
    let nums = vec![1,3,5,6,7,10,12,9,22,24,31,32,35];
    let res = Solution::largest_divisible_subset(nums);
    for t in res {
        println!("{}", t);
    }
}
