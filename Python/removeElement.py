
from typing import List
class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        for i in range(len(nums)):
            if val in nums:
                nums.remove(val)
        return len(nums)

nums:list[int]=[3,2,2,3]
print(Solution.removeElement(0,nums,2))