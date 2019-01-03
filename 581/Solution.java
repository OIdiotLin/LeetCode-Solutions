class Solution {
    public int findUnsortedSubarray(int[] nums) {
        int subArrL = 0;
        int subArrR = -1;
        int ptrL = 0;
        int ptrR = nums.length-1;
        int maxItem = Integer.MIN_VALUE;
        int minItem = Integer.MAX_VALUE;

        while (ptrR >= 0) {
            maxItem = Math.max(maxItem, nums[ptrL]);
            if (maxItem != nums[ptrL]) {
                subArrR = ptrL;
            }

            minItem = Math.min(minItem, nums[ptrR]);
            if (minItem != nums[ptrR]) {
                subArrL = ptrR;
            }
            
            ptrL += 1;
            ptrR -= 1;
        }

        return subArrR - subArrL + 1;
    }

    public static void main(String[] args) {
        int[] nums = new int[]{2, 6, 4, 8, 10, 9, 15};
        System.out.println(new Solution().findUnsortedSubarray(nums));
    }
}