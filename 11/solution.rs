struct Solution {}
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = (height.len() - 1) as i32;
        let mut ans = 0;
        while l != r {
            let l_height = &height[l as usize];
            let r_height = &height[r as usize];
            if l_height < r_height {
                ans = std::cmp::max(ans, l_height * (r - l));
                l += 1;
            } else {
                ans = std::cmp::max(ans, r_height * (r - l));
                r -= 1;
            }
        }
        ans
    }
}

fn main() {
    let h = vec![1,8,6,2,5,4,8,3,7];
    println!("{}", Solution::max_area(h));
}