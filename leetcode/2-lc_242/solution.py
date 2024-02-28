class Solution:
  def isAnagram(self, s: str, t: str) -> bool:
    if len(s) != len(t):
      return False
    
    s_count = [0] * 26
    t_count = [0] * 26

    for c in s:
      s_count[ord(c) - ord('a')] += 1

    for c in t:
      t_count[ord(c) - ord('a')] += 1
    
    return s_count == t_count
