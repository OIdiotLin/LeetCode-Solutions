/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.
 * 
 * Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
 * 
 * You may assume nums1 and nums2 cannot be both empty.
 * 
 * Example 1:
 * 
 * 
 * nums1 = [1, 3]
 * nums2 = [2]
 * 
 * The median is 2.0
 * 
 * 
 * Example 2:
 * 
 * 
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 * 
 * The median is (2 + 3)/2 = 2.5
 * 
 * 
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n, a, b) = if nums1.len() < nums2.len() {
            (nums1.len(), nums2.len(), nums1, nums2)
        } else {
            (nums2.len(), nums1.len(), nums2, nums1)
        };

        let mut l = 0usize;
        let mut r = m;
        let mut mid = (m + n + 1) / 2;
        while l <= r {
            let i = (l + r) / 2;
            let j = mid - i;
            if i < r && b[j - 1] > a[i] {
                l = i + 1;
            } else if i > l && a[i - 1] > b[j] {
                r = i - 1;
            } else {
                let mut single_l = 0f64;
                if i == 0usize {
                    single_l = b[j - 1] as f64;
                } else if j == 0usize {
                    single_l = a[i - 1] as f64;
                } else {
                    single_l = std::cmp::max(a[i - 1], b[j - 1]) as f64;
                }

                if (m + n) % 2 == 1 {
                    return single_l;
                }

                let mut single_r = 0f64;
                if i == m {
                    single_r = b[j] as f64;
                } else if j == n {
                    single_r = a[i] as f64;
                } else {
                    single_r = std::cmp::min(a[i], b[j]) as f64;
                }

                return (single_l + single_r) / 2f64;
            }
        }
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}