import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 8
dat = open(f'../input/day{day}.txt').read().split('\n')
grid = [list(x) for x in dat]
n, m = len(grid), len(grid[0])

# part 1
antennas = {}
for i in range(n):
    for j in range(m):
        if grid[i][j] != '.':
            c = grid[i][j]
            antennas[c] = antennas.get(c, []) + [(i, j)]
    
# print(antennas)

antinodes = set()
for antenna in antennas:
    locs = antennas[antenna]
    l = len(locs)
    for i in range(l):
        a = locs[i]
        for j in range(i + 1, l):
            b = locs[j]
            dx, dy = b[0] - a[0], b[1] - a[1]

            x, y = a 
            while 0 <= x < n and 0 <= y < m:
                da = a[0] - x, a[1] - y
                db = b[0] - x, b[1] - y
                if da[0] * 2 == db[0] and da[1] * 2 == db[1]:
                    antinodes.add((x, y))
                elif db[0] * 2 == da[0] and db[1] * 2 == da[1]:
                    antinodes.add((x, y))
                x += dx
                y += dy
            
            x, y = a 
            while 0 <= x < n and 0 <= y < m:
                da = a[0] - x, a[1] - y
                db = b[0] - x, b[1] - y
                if da[0] * 2 == db[0] and da[1] * 2 == db[1]:
                    antinodes.add((x, y))
                elif db[0] * 2 == da[0] and db[1] * 2 == da[1]:
                    antinodes.add((x, y))
                x -= dx
                y -= dy

print(len(antinodes))


# part 2

antinodes = set()
for antenna in antennas:
    locs = antennas[antenna]
    l = len(locs)
    for i in range(l):
        a = locs[i]
        for j in range(i + 1, l):
            b = locs[j]
            dx, dy = b[0] - a[0], b[1] - a[1]

            x, y = a 
            while 0 <= x < n and 0 <= y < m:
                da = a[0] - x, a[1] - y
                db = b[0] - x, b[1] - y
                antinodes.add((x, y))
                x += dx
                y += dy
            
            x, y = a 
            while 0 <= x < n and 0 <= y < m:
                da = a[0] - x, a[1] - y
                db = b[0] - x, b[1] - y
                antinodes.add((x, y))
                x -= dx
                y -= dy

print(len(antinodes))
