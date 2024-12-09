import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 7
dat = open(f'../input/day{day}.txt').read().splitlines()

# part 1
s = 0
for row in dat:
    res, nums = row.split(': ')
    res = int(res)
    nums = list(map(int, nums.split()))
    
    possible = set([nums[0]])
    for i in range(1, len(nums)):
        new = set()
        for p in possible:
            new.add(p + nums[i])
            new.add(p * nums[i])
        new = set([x for x in new if x <= res])
        possible = new

    if res in possible:
        s += res

print(s)

# part 2
s = 0
for row in dat:
    res, nums = row.split(': ')
    res = int(res)
    nums = list(map(int, nums.split()))
    
    possible = set([nums[0]])
    for i in range(1, len(nums)):
        new = set()
        for p in possible:
            new.add(p + nums[i])
            new.add(p * nums[i])
            new.add(int(str(p) + str(nums[i]))) 
        new = set([x for x in new if x <= res])
        possible = new

    if res in possible:
        s += res

print(s)


