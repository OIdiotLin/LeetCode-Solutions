struct Solution {}
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        for n in nums {
            xor = xor ^ n;
        }
        return xor;
    }

    pub fn single_number_cooler(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |ret, x| ret ^ x)
    }
}

fn main () {
    let res = Solution::single_number(vec![4,1,2,3,3,2,4]);
    println!("{}", res)
}
