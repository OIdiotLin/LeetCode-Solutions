struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut l2r_prod = *(nums.first().unwrap_or(&0));
        let mut r2l_prod = *(nums.last().unwrap_or(&0));
        let mut l2r_max = l2r_prod;
        let mut r2l_max = r2l_prod;
        for n in nums.iter().skip(1) {
            l2r_prod = n * match l2r_prod {0 => 1, _ => l2r_prod}; 
            l2r_max = std::cmp::max(l2r_max, l2r_prod);
        }
        for n in nums.iter().rev().skip(1) {
            r2l_prod = n * match r2l_prod {0 => 1, _ => r2l_prod}; 
            r2l_max = std::cmp::max(r2l_max, r2l_prod);
        }
        std::cmp::max(l2r_max, r2l_max)
    }
}


fn main() {
    let nums = vec![-2, 0, -1];
    println!("{}", Solution::max_product(nums));
}
