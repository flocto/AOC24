day = 6
dat = open(f'../input/day{day}.txt').read().splitlines()
grid = [list(x) for x in dat]

# part 1
start = [(x, y) for x in range(len(grid)) for y in range(len(grid[0])) if grid[x][y] == '^'][0]
dxy = [(-1, 0), (0, 1), (1, 0), (0, -1)]

def sim(start, grid):
    d = 0
    pos = set()
    pos.add(start)
    x, y = start
    while True:
        nx, ny = x + dxy[d][0], y + dxy[d][1]
        if nx < 0 or nx >= len(grid) or ny < 0 or ny >= len(grid[0]):
            break
        if grid[nx][ny] == '#':
            d = (d + 1) % 4
            continue
        x, y = nx, ny
        pos.add((nx, ny))
    return pos

path = sim(start, grid)
print(len(path))

# part 2
def sim2(start, grid):
    d = 0
    pos = set()
    pos.add((start[0], start[1], d))
    x, y = start
    inf = False
    while True:
        nx, ny = x + dxy[d][0], y + dxy[d][1]
        if nx < 0 or nx >= len(grid) or ny < 0 or ny >= len(grid[0]):
            break
        if grid[nx][ny] == '#':
            d = (d + 1) % 4
            continue
        x, y = nx, ny
        if (nx, ny, d) in pos:
            inf = True
            break
        pos.add((nx, ny, d))
    return pos, inf

c = 0
path = sim(start, grid)
for x, y in path:
    if grid[x][y] != '.': # i mean this only happens on the spawn square and you can't put blocker there anyway
        continue

    t_grid = [list(x) for x in grid]
    t_grid[x][y] = '#'
    c += sim2(start, t_grid)[1]

print(c)