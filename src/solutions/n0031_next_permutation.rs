/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 * 
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 * 
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and use only constant extra memory.
 * 
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 * 
 * 1,2,3 &rarr; 1,3,2<br />
 * 3,2,1 &rarr; 1,2,3<br />
 * 1,1,5 &rarr; 1,5,1
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut k = nums.len() - 1;
        while k > 0 && nums[k] <= nums[k - 1] {
            k -= 1;
        }
        if k == 0 {     // got max permutation
            nums.reverse();
            return;
        }
        k = k - 1;
        let mut u = nums.len() - 1;
        while u > k && nums[u] <= nums[k] {
            u -= 1;
        }
        nums.swap(k, u);
        Solution::reverse(nums, k + 1, nums.len() - 1);
    }

    fn reverse(arr: &mut Vec<i32>, begin: usize, end: usize) {
        let mut i = begin;
        let mut j = end;
        while i < j {
            arr.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31_case0() {
        let mut nums = vec![1, 3, 5, 4, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 4, 2, 3, 5])
    }

    #[test]
    fn test_31_case1() {
        let mut nums = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_31_case2() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }
}