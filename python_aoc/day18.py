import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re
from heapq import heapify, heappush, heappop
from tqdm import tqdm

day = 18
inp = open(f'../input/day{day}.txt').read().splitlines()
inp = [tuple(map(int, l.split(','))) for l in inp]
n, m = 71, 71
grid = [['.' for _ in range(m)] for _ in range(n)]

# part 1
dxy = [(0, 1), (0, -1), (1, 0), (-1, 0)]
for i in range(1024):
    x, y = inp[i]
    grid[x][y] = '#'

start = 0, 0
end = 70, 70
q = deque([start])
dist = defaultdict(lambda: math.inf)
dist[start] = 0
while q:
    x, y = q.popleft()
    if (x, y) == end:
        print(dist[(x, y)])
        break

    for dx, dy in dxy:
        nx, ny = x + dx, y + dy
        if 0 <= nx < n and 0 <= ny < m and grid[nx][ny] == '.' and dist[(nx, ny)] == math.inf:
            dist[(nx, ny)] = dist[(x, y)] + 1
            q.append((nx, ny))
    


# part 2
def check_possible(grid):
    start = 0, 0
    end = n - 1, m - 1
    q = deque([start])
    dist = defaultdict(lambda: math.inf)
    dist[start] = 0
    while q:
        x, y = q.popleft()
        if (x, y) == end:
            break

        for dx, dy in dxy:
            nx, ny = x + dx, y + dy
            if 0 <= nx < n and 0 <= ny < m and grid[nx][ny] == '.' and dist[(nx, ny)] == math.inf:
                dist[(nx, ny)] = dist[(x, y)] + 1
                q.append((nx, ny))

    return dist[end] != math.inf

l, r = 1024, len(inp)
while l < r:
    mid = (l + r) // 2
    grid = [['.' for _ in range(m)] for _ in range(n)]
    for i in range(mid):
        x, y = inp[i]
        grid[x][y] = '#'

    if check_possible(grid):
        l = mid + 1
    else:
        r = mid

print(inp[l - 1])