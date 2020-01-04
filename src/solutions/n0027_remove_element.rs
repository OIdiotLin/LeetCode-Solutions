/**
 * [27] Remove Element
 *
 * Given an array nums and a value val, remove all instances of that value <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and return the new length.
 * 
 * Do not allocate extra space for another array, you must do this by modifying the input array <a href="https://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> with O(1) extra memory.
 * 
 * The order of elements can be changed. It doesn't matter what you leave beyond the new length.
 * 
 * Example 1:
 * 
 * 
 * Given nums = [3,2,2,3], val = 3,
 * 
 * Your function should return length = 2, with the first two elements of nums being 2.
 * 
 * It doesn't matter what you leave beyond the returned length.
 * 
 * 
 * Example 2:
 * 
 * 
 * Given nums = [0,1,2,2,3,0,4,2], val = 2,
 * 
 * Your function should return length = 5, with the first five elements of nums containing 0, 1, 3, 0, and 4.
 * 
 * Note that the order of those five elements can be arbitrary.
 * 
 * It doesn't matter what values are set beyond the returned length.
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
 * int len = removeElement(nums, val);
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
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut unique_len = 0usize;
            for i in 0..nums.len() {
                if nums[i] != val {
                    nums[unique_len] = nums[i];
                    unique_len += 1;
                }
            }
            unsafe {
                nums.set_len(unique_len);
            }
            unique_len as i32
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27_case0() {
        let mut x = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut x, 3), 2);
        assert_eq!(x, vec![2, 2]);
    }

    #[test]
    fn test_27_case1() {
        let mut x = vec![1, 1, 2, 3, 1, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut x, 1), 5);
        assert_eq!(x, vec![2, 3, 2, 2, 3]);
    }
}