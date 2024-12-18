import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re
from tqdm import *

day = 12
inp = open(f'../input/day{day}.txt').read().splitlines()
grid = [list(x) for x in inp]
n, m = len(grid), len(grid[0])

# part 1
dxy = [(0, 1), (1, 0), (0, -1), (-1, 0)]
visited = set()
def dfs(cur, c):
    global visited
    visited.add(cur)

    perim, area = 0, 0
    for dx, dy in dxy:
        nx, ny = cur[0] + dx, cur[1] + dy
        if 0 <= nx < n and 0 <= ny < m:
            if grid[nx][ny] == c and (nx, ny) not in visited:
                p, a = dfs((nx, ny), c)
                perim += p
                area += a
            elif grid[nx][ny] != c:
                perim += 1
        else:
            perim += 1

    return perim, area + 1

ans = 0
for i in range(n):
    for j in range(m):
        if (i, j) not in visited:
            p, a = dfs((i, j), grid[i][j])
            # print(grid[i][j], p, a)
            ans += a * p

print(ans)
# part 2
# pad grid with walls
grid = [['#'] * (m + 2)] + [['#'] + x + ['#'] for x in grid] + [['#'] * (m + 2)]
n, m = len(grid), len(grid[0])
visited = set()
def subbfs(cur, c, fxy):
    visited = set()
    visited.add(cur)
    fx, fy = fxy

    q = deque([cur])
    while q:
        x, y = q.popleft()
        for dx, dy in dxy:
            nx, ny = x + dx, y + dy
            if 0 <= nx < n and 0 <= ny < m:
                # print(nx + fx, ny + fy, grid[nx][ny], grid[nx + fx][ny + fy])
                if grid[nx][ny] != c and grid[nx + fx][ny + fy] == c and (nx, ny) not in visited:
                    visited.add((nx, ny))
                    q.append((nx, ny))

    return visited

def dfs(cur, c):
    global visited
    visited.add(cur)

    corners, perim, area = set(), set(), 0
    for dx, dy in dxy:
        nx, ny = cur[0] + dx, cur[1] + dy
        if 0 <= nx < n and 0 <= ny < m:
            if grid[nx][ny] == c and (nx, ny) not in visited:
                cor, p, a = dfs((nx, ny), c)
                corners |= cor
                perim |= p
                area += a
            elif grid[nx][ny] != c:
                # if (nx, ny) in perim:
                #     continue
                edge = subbfs((nx, ny), c, (-dx, -dy))
                corners.add((list(sorted(edge))[0], (-dx, -dy)))

    return corners, perim, area + 1

ans = 0
for i in range(1, n - 1):
    for j in range(1, m - 1):
        if (i, j) not in visited:
            c, p, a = dfs((i, j), grid[i][j])
            
            # print(grid[i][j], a, len(c), sorted(c), a * len(c))
            ans += a * len(c)

print(ans)
