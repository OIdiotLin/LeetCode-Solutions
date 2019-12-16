/**
 * [15] 3Sum
 *
 * Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
 * Find all unique triplets in the array which gives the sum of zero.
 * 
 * Note:
 * 
 * The solution set must not contain duplicate triplets.
 * 
 * Example:
 * 
 * 
 * Given array nums = [-1, 0, 1, 2, -1, -4],
 * 
 * A solution set is:
 * [
 *   [-1, 0, 1],
 *   [-1, -1, 2]
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut sorted = nums;
        sorted.sort();
        println!("{:#?}", sorted);
        let mut res = vec![];

        for i in 0..sorted.len() {
            if sorted[i] > 0 {
                break;
            }
            if i > 0 && sorted[i - 1] == sorted[i] {
                continue;
            }
            let mut l: usize = i + 1;
            let mut r: usize = sorted.len() - 1;
            while l < r {
                let temp_sum = sorted[i] + sorted[l] + sorted[r];
                if temp_sum == 0 {
                    res.push(vec![sorted[i], sorted[l], sorted[r]]);
                    while l < r && sorted[l] == sorted[l + 1] {
                        l += 1;
                    }
                    while l < r && sorted[r] == sorted[r - 1] {
                        r -= 1;
                    }
                    l += 1;
                    r -= 1;
                } else if temp_sum > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
        assert_eq!(Solution::three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}