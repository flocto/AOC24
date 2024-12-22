import math
import itertools
from functools import cache
from collections import Counter, defaultdict, deque
import bisect
import re
from heapq import heapify, heappush, heappop
from tqdm import tqdm

day = 21
inp = open(f'../input/day{day}.txt').read().splitlines()

numpad = [
    '789',
    '456',
    '123',
    ' 0A'
]
arrows = [
    ' ^A',
    '<v>',
]

num_coords = {
    numpad[i][j]: (j, i) for i in range(4) for j in range(3) if numpad[i][j] != ' '
}
arrows_coords = {
    arrows[i][j]: (j, i) for i in range(2) for j in range(3) if arrows[i][j] != ' '
}

dp = []
def get_presses(insts, cost=None):
    if not cost:
        cost = dp[-1]
    insts = 'A' + insts
    return sum(cost[(insts[i], insts[i+1])] for i in range(len(insts) - 1))

dp.append({})
for c1 in arrows_coords:
    for c2 in arrows_coords:
        dp[0][(c1, c2)] = 1

# part 1
for _ in range(2):
    next_dp = {}
    for c1, (x1, y1) in arrows_coords.items():
        for c2, (x2, y2) in arrows_coords.items():
            horiz = '<>'[x2 > x1] * abs(x2 - x1)
            vert = '^v'[y2 > y1] * abs(y2 - y1)
            horiz_first = get_presses(horiz + vert + 'A')
            vert_first = get_presses(vert + horiz + 'A')

            # edge case
            if c1 in '^A' and c2 == '<':
                horiz_first = float('inf')
            if c1 == '<' and c2 in '^A':
                vert_first = float('inf')
            
            # print(len(dp), c1, c2, horiz_first, vert_first)
            next_dp[(c1, c2)] = min(horiz_first, vert_first)
    dp.append(next_dp)

c = 0
for code in inp:
    code = 'A' + code
    s = 0
    for i in range(len(code) - 1):
        c1, c2 = code[i], code[i+1]
        x1, y1 = num_coords[c1]
        x2, y2 = num_coords[c2]

        horiz = '<>'[x2 > x1] * abs(x2 - x1)
        vert = '^v'[y2 > y1] * abs(y2 - y1)

        horiz_first = get_presses(horiz + vert + 'A')
        vert_first = get_presses(vert + horiz + 'A')

        # edge case
        if c1 in '0A' and c2 in '741':
            horiz_first = float('inf')
        if c1 in '741' and c2 in '0A':
            vert_first = float('inf')
        
        # print(c1, c2, horiz_first, vert_first)
        s += min(horiz_first, vert_first)
    c += s * int(code[1:-1])
print(c)

# part 2
for _ in range(23):
    next_dp = {}
    for c1, (x1, y1) in arrows_coords.items():
        for c2, (x2, y2) in arrows_coords.items():
            horiz = '<>'[x2 > x1] * abs(x2 - x1)
            vert = '^v'[y2 > y1] * abs(y2 - y1)
            horiz_first = get_presses(horiz + vert + 'A')
            vert_first = get_presses(vert + horiz + 'A')

            # edge case
            if c1 in '^A' and c2 == '<':
                horiz_first = float('inf')
            if c1 == '<' and c2 in '^A':
                vert_first = float('inf')
            
            # print(len(dp), c1, c2, horiz_first, vert_first)
            next_dp[(c1, c2)] = min(horiz_first, vert_first)
    dp.append(next_dp)
    
c = 0
for code in inp:
    code = 'A' + code
    s = 0
    for i in range(len(code) - 1):
        c1, c2 = code[i], code[i+1]
        x1, y1 = num_coords[c1]
        x2, y2 = num_coords[c2]

        horiz = '<>'[x2 > x1] * abs(x2 - x1)
        vert = '^v'[y2 > y1] * abs(y2 - y1)

        horiz_first = get_presses(horiz + vert + 'A')
        vert_first = get_presses(vert + horiz + 'A')

        # edge case
        if c1 in '0A' and c2 in '741':
            horiz_first = float('inf')
        if c1 in '741' and c2 in '0A':
            vert_first = float('inf')
        
        # print(c1, c2, horiz_first, vert_first)
        s += min(horiz_first, vert_first)
    c += s * int(code[1:-1])

print(c)