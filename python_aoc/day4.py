import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 4
dat = open(f'../input/day{day}.txt').read().splitlines()

# part 1
c = 0
dxy = [(x, y) for x in range(-1, 2) for y in range(-1, 2)]
for a in range(len(dat)):
    for b in range(len(dat[0])):
        if dat[a][b] != 'X':
            continue

        for dx, dy in dxy:
            s = 'X'
            x, y = a, b
            for _ in range(3):
                x, y = x + dx, y + dy
                if 0 <= x < len(dat) and 0 <= y < len(dat[0]):
                    s += dat[x][y]
            if s == 'XMAS':
                c += 1
print(c)

# part 2
c = 0
for a in range(1, len(dat) - 1):
    for b in range(1, len(dat[0]) - 1):
        block = [dat[a + x][b + y] for x, y in dxy]
        if block[4] != 'A':
            continue

        down = ''.join(block[::4])
        up = ''.join(block[2:7:2])
        if down in ['MAS', 'SAM'] and up in ['MAS', 'SAM']:
            c += 1

print(c)