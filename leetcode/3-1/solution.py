class Solution:
  def twoSum(self, nums: List[int], target: int) -> List[int]:
    unique = dict()

    for index, value in enumerate(nums):
      diff = target - value

      if diff in unique:
        return [unique[diff], index]
      
      unique[value] = index
    
    return
