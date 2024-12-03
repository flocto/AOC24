import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 3
dat = open(f'../input/day{day}.txt').read()

# part 1
p = 0
for match in re.findall(r'mul\(\d+,\d+\)', dat):
    a, b = map(int, match.split('(')[1][:-1].split(','))
    p += a * b
print(p)

# part 2
p = 0
active = True
for m in re.findall(r'mul\(\d+,\d+\)|do\(\)|don\'t\(\)', dat):
    match m:
        case 'do()':
            active = True
        case 'don\'t()':
            active = False
        case _:
            if active:
                a, b = map(int, m.split('(')[1][:-1].split(','))
                p += a * b
print(p)