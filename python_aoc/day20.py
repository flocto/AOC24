from collections import deque
from tqdm import trange

day = 20
inp = open(f'../input/day{day}.txt').read().split('\n')
grid = [list(x) for x in inp]
dxy = [(0, 1), (0, -1), (1, 0), (-1, 0)]
n, m = len(grid), len(grid[0])

sx, sy = [(x, y) for x in range(n) for y in range(m) if grid[x][y] == 'S'][0]
ex, ey = [(x, y) for x in range(n) for y in range(m) if grid[x][y] == 'E'][0]

def precalc(grid, start):
    dist = [[-1]*m for _ in range(n)]
    dist[start[0]][start[1]] = 0
    q = deque([start])
    while q:
        x, y = q.popleft()
        for dx, dy in dxy:
            nx, ny = x + dx, y + dy
            if 0 <= nx < n and 0 <= ny < m and grid[nx][ny] != '#' and dist[nx][ny] == -1:
                dist[nx][ny] = dist[x][y] + 1
                q.append((nx, ny))

    return dist

start_dist = precalc(grid, (sx, sy))
base = start_dist[ex][ey]
end_dist = precalc(grid, (ex, ey))

def bfs(grid, start, cap=20):
    visited = set()
    q = deque([(start, 0)]) 
    visited.add((start[0], start[1], 0))
    endings = set()

    while q:
        (x, y), cheated = q.popleft()
        if cheated and grid[x][y] != '#' and (x, y, cheated) not in endings:
            endings.add((x, y, cheated))
    
        if cheated == cap:
            continue

        for dx, dy in dxy:
            nx, ny = x + dx, y + dy
            if 0 <= nx < n and 0 <= ny < m and (nx, ny, cheated + 1) not in visited:
                # visited[nx, ny, cheated + 1] = visited[x, y, cheated] + 1
                visited.add((nx, ny, cheated + 1))
                q.append(((nx, ny), cheated + 1))

    return endings

# part 1
cnt = set()
for x in trange(1, n-1):
    for y in range(1, m-1):
        if start_dist[x][y] != -1:
            for (cx, cy, cheat) in bfs(grid, (x, y), cap = 2):
                if end_dist[cx][cy] != -1:
                    length = start_dist[x][y] + end_dist[cx][cy] + cheat
                    if base - length >= 100:
                        cnt.add((x, y, cx, cy))

print(len(cnt))

# part 2
cnt = set()
for x in trange(1, n-1):
    for y in range(1, m-1):
        if start_dist[x][y] != -1:
            for (cx, cy, cheat) in bfs(grid, (x, y)):
                if end_dist[cx][cy] != -1:
                    length = start_dist[x][y] + end_dist[cx][cy] + cheat
                    if base - length >= 100:
                        cnt.add((x, y, cx, cy))

print(len(cnt))