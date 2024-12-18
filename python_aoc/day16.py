import math
from collections import deque
import bisect

day = 16
inp = open(f'../input/day{day}.txt').read().splitlines()
grid = [list(x) for x in inp]
dxy = [(0, 1), (1, 0), (0, -1), (-1, 0)]

n, m = len(grid), len(grid[0])
sx, sy = [(x, y) for x in range(n) for y in range(m) if grid[x][y] == 'S'][0]
ex, ey = [(x, y) for x in range(n) for y in range(m) if grid[x][y] == 'E'][0]

def insert(q: deque, x, y, d, s, path):
    idx = bisect.bisect_left(q, s, key=lambda x: x[3])
    q.insert(idx, (x, y, d, s, path))

q = deque([(sx, sy, 0, 0, [(sx, sy)])])
best_path = {(sx, sy, 0): (0, set())}
while q:
    x, y, d, s, path = q.popleft()

    if x < 0 or x >= n or y < 0 or y >= m:
        continue

    if (x, y, d) in best_path:
        if s > best_path[(x, y, d)][0]:
            if (x, y) == (ex, ey):
                break
            continue
        else:
            on_best_path = best_path[(x, y, d)][1]
            on_best_path |= set(path)
            best_path[(x, y, d)] = (s, on_best_path)
    else:
        best_path[(x, y, d)] = (s, set(path))

    if grid[x][y] == 'E':
        continue

    nx, ny = x + dxy[d][0], y + dxy[d][1]
    if grid[nx][ny] != '#': 
        insert(q, nx, ny, d, s+1, path + [(nx, ny)])

    insert(q, x, y, (d+1)%4, s+1000, path)
    insert(q, x, y, (d-1)%4, s+1000, path)

best = (math.inf, set())
for d in range(4):
    if (ex, ey, d) in best_path:
        s, path = best_path[(ex, ey, d)]
        if s < best[0]:
            best = (s, path)

print(best[0], len(best[1]))