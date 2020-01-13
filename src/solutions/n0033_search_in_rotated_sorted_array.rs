/**
 * [33] Search in Rotated Sorted Array
 *
 * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
 * 
 * (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
 * 
 * You are given a target value to search. If found in the array return its index, otherwise return -1.
 * 
 * You may assume no duplicate exists in the array.
 * 
 * Your algorithm's runtime complexity must be in the order of O(log n).
 * 
 * Example 1:
 * 
 * 
 * Input: nums = [4,5,6,7,0,1,2], target = 0
 * Output: 4
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: nums = [4,5,6,7,0,1,2], target = 3
 * Output: -1
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut l = 0;
        let mut r = (nums.len() - 1) as i32;
        while l <= r {
            let mid = l + ((r - l) >> 1);
            if nums[mid as usize] == target {
                return mid as i32;
            }
            if nums[mid as usize] < nums[r as usize] {
                if nums[mid as usize] < target && target <= nums[r as usize] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            } else {
                if nums[mid as usize] > target && target >= nums[l as usize] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            }
        }
        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_33_case0() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }

    #[test]
    fn test_33_case1() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }

    #[test]
    fn test_33_case2() {
        assert_eq!(Solution::search(vec![], 3), -1);
    }

    #[test]
    fn test_33_case3() {
        assert_eq!(Solution::search(vec![1, 3], 0), -1);
    }

    #[test]
    fn test_33_case4() {
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}