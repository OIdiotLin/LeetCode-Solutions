/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 * 
 * Your algorithm's runtime complexity must be in the order of O(log n).
 * 
 * If the target is not found in the array, return [-1, -1].
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        vec![Solution::search_first(&nums, target), Solution::search_last(&nums, target)]
    }

    fn search_first(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0usize;
        let mut r = nums.len() - 1;
        while l < r {
            let mid = l + ((r - l) >> 1);
            if nums[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if nums[l] == target { l as i32 } else { -1 }
    }

    fn search_last(nums: &Vec<i32>, target: i32) -> i32 {
        let mut l = 0usize;
        let mut r = nums.len() - 1;
        while (l as i32) < ((r as i32) - 1) {
            let mid = l + ((r - l) >> 1);
            if nums[mid] <= target {
                l = mid;
            } else {
                r = mid;
            }
        }
        if nums[r] == target { r as i32 } else if nums[l] == target { l as i32 } else { -1 }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34_case0() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    }

    #[test]
    fn test_34_case1() {
        assert_eq!(Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6), vec![-1, -1]);
    }
}