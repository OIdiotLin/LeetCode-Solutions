use std::cmp;

struct Solution {}
impl Solution {
    /**
     * Ans = max(max(0, f_i + a_i))
     */
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // if positive nums do not exist, return the largest num
        let largest = nums.iter().fold(nums[0], |t, x| cmp::max(t, *x));
        if largest <= 0 {
            return largest;
        }

        let mut ans = 0;
        let mut f = 0;
        for n in nums {
            f = cmp::max(0, f + n);
            ans = cmp::max(ans, f);
        }
        ans
    }
}


fn main () {
    let res = Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]);
    println!("{}", res);
} 
