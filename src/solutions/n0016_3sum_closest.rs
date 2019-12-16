/**
 * [16] 3Sum Closest
 *
 * Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target. Return the sum of the three integers. You may assume that each input would have exactly one solution.
 * 
 * Example:
 * 
 * 
 * Given array nums = [-1, 2, 1, -4], and target = 1.
 * 
 * The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut sorted = nums;
        sorted.sort();
        let mut opt_sum = 0xFFFF;

        for i in 0..sorted.len() {
            if i > 0 && sorted[i] == sorted[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = sorted.len() - 1;
            while l < r {
                let temp_sum = sorted[i] + sorted[l] + sorted[r];
                if temp_sum == target {
                    return temp_sum;
                }
                if (temp_sum - target).abs() < (opt_sum - target).abs() {
                    opt_sum = temp_sum;
                }

                if temp_sum - target < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        opt_sum
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}