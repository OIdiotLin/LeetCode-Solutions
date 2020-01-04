/**
 * [26] Remove Duplicates from Sorted Array
 *
 * Given a sorted array nums, remove the duplicates <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> such that each element appear only once and return the new length.
 * 
 * Do not allocate extra space for another array, you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * 
 * Example 1:
 * 
 * 
 * Given nums = [1,1,2],
 * 
 * Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
 * 
 * It doesn't matter what you leave beyond the returned length.
 * 
 * Example 2:
 * 
 * 
 * Given nums = [0,0,1,1,1,2,2,3,3,4],
 * 
 * Your function should return length = 5, with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.
 * 
 * It doesn't matter what values are set beyond the returned length.
 * 
 * 
 * Clarification:
 * 
 * Confused why the returned value is an integer but your answer is an array?
 * 
 * Note that the input array is passed in by reference, which means modification to the input array will be known to the caller as well.
 * 
 * Internally you can think of this:
 * 
 * 
 * // nums is passed in by reference. (i.e., without making a copy)
 * int len = removeDuplicates(nums);
 * 
 * // any modification to nums in your function would be known by the caller.
 * // using the length returned by your function, it prints the first len elements.
 * for (int i = 0; i < len; i++) {
 *     print(nums[i]);
 * }
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            nums.len() as i32
        } else {
            let mut slow_idx = 0usize;
            let mut fast_idx = 1usize;
            while fast_idx < nums.len() {
                if nums[slow_idx] != nums[fast_idx] {
                    slow_idx += 1;
                    nums[slow_idx] = nums[fast_idx];
                }
                fast_idx += 1;
            }
            unsafe {
                nums.set_len(slow_idx + 1);
            }
            (slow_idx + 1) as i32
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_26_case0() {
        let mut x = vec![];
        assert_eq!(Solution::remove_duplicates(&mut x), 0);
        assert_eq!(x, vec![]);
    }

    #[test]
    fn test_26_case1() {
        let mut x = vec![1];
        assert_eq!(Solution::remove_duplicates(&mut x), 1);
        assert_eq!(x, vec![1]);
    }

    #[test]
    fn test_26_case2() {
        let mut x = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut x), 2);
        assert_eq!(x, vec![1, 2]);
    }

    #[test]
    fn test_26_case3() {
        let mut x = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut x), 5);
        assert_eq!(x, vec![0, 1, 2, 3, 4]);
    }
}