class Solution:
    def _isEmail(self, S):
        return '@' in S
    
    def _dealEmail(self, S):
        S = S.lower()
        username, domain = S.split('@')
        return '{}{}{}@{}'.format(username[0], '*'*5, username[-1], domain)

    def _dealPhone(self, S):
        nums = list(filter(lambda x: x.isdigit(), S))
        if len(nums) == 10:
            return self._dealLocalNumber(nums)
        else:
            return '{}-{}'.format(self._dealInternationalNumber(nums[0: -10]),
                                  self._dealLocalNumber(nums[-10:]))

    def _dealLocalNumber(self, nums):
        return '{0}-{0}-{1}'.format('*'*3, ''.join(nums[-4:]))
    
    def _dealInternationalNumber(self, nums):
        return '+{}'.format('*'*len(nums))

    def maskPII(self, S):
        """
        :type S: str
        :rtype: str
        """
        if self._isEmail(S):
            return self._dealEmail(S)
        else:
            return self._dealPhone(S)
        

if __name__ == "__main__":
    s = Solution()
    print(s.maskPII('1(234)567-890'))
    print(s.maskPII('86-(10)12345678'))
    print(s.maskPII('LeetCode@LeetCode.com'))
