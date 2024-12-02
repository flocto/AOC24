import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 2
dat = open(f'../input/day{day}.txt').read().splitlines()

# part 1
s = 0
for line in dat:
    report = list(map(int, line.split()))

    srep = sorted(report)
    if srep == report or srep == report[::-1]:
        if all(1 <= abs(srep[i] - srep[i-1]) <= 3 for i in range(1, len(srep))):
            s += 1
    
print(s)

# part 2
s = 0
for line in dat:
    report = list(map(int, line.split()))
    safe = False
    srep = sorted(report)
    if srep == report or srep == report[::-1]:
        if all(1 <= abs(srep[i] - srep[i-1]) <= 3 for i in range(1, len(srep))):
            s += 1
            safe = True
    
    if not safe:
        for i in range(len(report)):
            rep = report[:i] + report[i+1:]
            srep = sorted(rep)
            if srep == rep or srep == rep[::-1]:
                if all(1 <= abs(srep[i] - srep[i-1]) <= 3 for i in range(1, len(srep))):
                    s += 1
                    break
    
print(s)