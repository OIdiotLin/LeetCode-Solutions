/**
 * [18] 4Sum
 *
 * Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target?
 * Find all unique quadruplets in the array which gives the sum of target.
 * 
 * Note:
 * 
 * The solution set must not contain duplicate quadruplets.
 * 
 * Example:
 * 
 * 
 * Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
 * 
 * A solution set is:
 * [
 *   [-1,  0, 0, 1],
 *   [-2, -1, 1, 2],
 *   [-2,  0, 0, 2]
 * ]
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        let len = nums.len();
        if len < 4 {
            return res;
        }

        let mut sorted = nums;
        sorted.sort();

        for idx_a in 0..len - 3 {
            if idx_a > 0 && sorted[idx_a] == sorted[idx_a - 1] { continue; }

            let cur_sum_min = sorted[idx_a] + sorted[idx_a + 1] + sorted[idx_a + 2] + sorted[idx_a + 3];
            if cur_sum_min > target { break; }
            let cur_sum_max = sorted[idx_a] + sorted[len - 1] + sorted[len - 2] + sorted[len - 3];
            if cur_sum_max < target { continue; }

            for idx_b in idx_a + 1..len - 2 {
                if idx_b > idx_a + 1 && sorted[idx_b] == sorted[idx_b - 1] { continue; }

                let mut idx_c = idx_b + 1;
                let mut idx_d = len - 1;

                let cur_sum_min = sorted[idx_a] + sorted[idx_b] + sorted[idx_c] + sorted[idx_c + 1];
                if cur_sum_min > target { continue; }
                let cur_sum_max = sorted[idx_a] + sorted[idx_b] + sorted[idx_d] + sorted[idx_d - 1];
                if cur_sum_max < target { continue; }

                while idx_c < idx_d {
                    let cur_sum = sorted[idx_a] + sorted[idx_b] + sorted[idx_c] + sorted[idx_d];
                    if cur_sum == target {
                        res.push(vec![sorted[idx_a], sorted[idx_b], sorted[idx_c], sorted[idx_d]]);
                        idx_c += 1;
                        while idx_c < idx_d && sorted[idx_c] == sorted[idx_c - 1] { idx_c += 1; }
                        idx_d -= 1;
                        while idx_c < idx_d && idx_b < idx_d && sorted[idx_d] == sorted[idx_d + 1] { idx_d -= 1; }
                    } else if cur_sum > target {
                        idx_d -= 1;
                    } else {
                        idx_c += 1;
                    }
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
    fn test_18_case1() {
        let mut case = vec![
            vec![-1, 0, 0, 1],
            vec![-2, -1, 1, 2],
            vec![-2, 0, 0, 2]
        ];
        case.sort();
        let mut out = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        out.sort();
        assert_eq!(out, case);
    }

    #[test]
    fn test_18_case2() {
        let case: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::four_sum(vec![-1, 0, -2, 6, 5], 1), case);
    }

    #[test]
    fn test_18_case3() {
        let case: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::four_sum(vec![-1, 0, -2, 5], 3), case);
    }
}