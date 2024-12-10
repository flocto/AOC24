import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 10
dat = open(f'../input/day{day}.txt').read().splitlines()
grid = [[int(c) for c in line] for line in dat]

# part 1 and 2
dxy = [(-1, 0), (1, 0), (0, -1), (0, 1)]
def calc_trailhead(grid, start):
    global dxy
    c = [0, 0]
    seen = {}
    q = deque([start])
    seen[start] = 1

    while q:
        x, y = q.popleft()
        if grid[x][y] == 9:
            c[0] += 1
            c[1] += seen[(x, y)]
        
        for dx, dy in dxy:
            nx, ny = x + dx, y + dy
            if 0 <= nx < len(grid) and 0 <= ny < len(grid[0]):
                if grid[nx][ny] == grid[x][y] + 1:
                    if (nx, ny) not in seen:
                        q.append((nx, ny))
                        seen[(nx, ny)] = seen[(x, y)]
                    else:
                        seen[(nx, ny)] += seen[(x, y)]
    
    return c

s1, s2 = calc_trailhead(grid, (0, 0))
for i in range(len(grid)):
    for j in range(len(grid[0])):
        if grid[i][j] == 0:
            c = calc_trailhead(grid, (i, j))
            s1 += c[0]
            s2 += c[1]

print(f'Part 1: {s1}\nPart 2: {s2}')
