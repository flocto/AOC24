import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 9
dat = open(f'../input/day{day}.txt').read()
dat = [int(x) for x in dat]

# part 1
n = sum(dat)
block = [0 for _ in range(len(dat))]
i = 0
files = 0
for j, d in enumerate(dat):
    if j % 2 == 1:
        block[i:i+d] = [-1] * d
        i += d
    else:
        block[i:i+d] = [files] * d
        i += d
        files += 1

l, r = block.index(-1), len(block) - 1
while l < r:
    while l < r and block[l] != -1:
        l += 1
    while r > l and block[r] == -1:
        r -= 1
    block[l], block[r] = block[r], block[l]

block = block[:l]
print(sum(i * b for i, b in enumerate(block) if b != -1))

# part 2
blocks = []
for i, d in enumerate(dat):
    if i % 2 == 1:
        blocks.append((-1, d))
    else:
        blocks.append((len(blocks) // 2, d))

r = len(blocks) - 1 
moved = set()
while r >= 0:
    # print(r)
    fi, fd = blocks[r]
    if fi == -1:
        r -= 1
        continue
    if fi in moved:
        r -= 1
        continue

    found = False
    for i in range(1, r):
        si, sd = blocks[i]
        if si != -1:
            continue    

        if sd >= fd:
            found = True
            break

    if not found:
        r -= 1
        continue

    moved.add(fi)
    
    blocks[i] = (-1, blocks[i][1] - fd)
    blocks[r] = (-1, fd)
    blocks.insert(i, (fi, fd))
    r -= 1 # r is always gap block here so we can move to next one

block = []
for i, d in blocks:
    if i == -1:
        block.extend([-1] * d)
    else:
        block.extend([i] * d)

print(sum(i * b for i, b in enumerate(block) if b != -1))