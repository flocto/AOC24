day = 15
inp = open(f'../input/day{day}.txt').read().split('\n\n', 1)
grid, insts = inp
dirs = {
    '^': (-1, 0),
    'v': (1, 0),
    '<': (0, -1),
    '>': (0, 1)
}

grid = [list(row) for row in grid.split('\n')]
insts = ''.join(insts.split('\n'))

# part 1
n, m = len(grid), len(grid[0])
rx, ry = [(x, y) for x in range(n) for y in range(m) if grid[x][y] == '@'][0]

def do_move(grid, x, y, dx, dy):
    fx, fy = x + dx, y + dy
    while 0 <= fx < n and 0 <= fy < m and grid[fx][fy] == 'O':
        fx, fy = fx + dx, fy + dy
    if grid[fx][fy] == '#':
        return grid, x, y
    
    assert grid[fx][fy] == '.'
    while 0 <= fx < n and 0 <= fy < m and grid[fx][fy] != '@':
        grid[fx][fy] = grid[fx - dx][fy - dy]
        fx, fy = fx - dx, fy - dy
    grid[fx][fy] = '.'
    return grid, x + dx, y + dy

def print_grid(grid):
    for row in grid:
        print(''.join(row))
    print()

grid1 = [row.copy() for row in grid]
for inst in insts:
    dx, dy = dirs[inst]
    grid1, rx, ry = do_move(grid1, rx, ry, dx, dy)
    # print_grid(grid)

s = 0
for x in range(n):
    for y in range(m):
        if grid1[x][y] == 'O':
            s += 100 * x + y

print(s)

# part 2
wide_grid = [[] for _ in range(n)]
for i in range(n):
    for j in range(m):
        match grid[i][j]:
            case '#':
                wide_grid[i].extend(['#', '#'])
            case 'O':
                wide_grid[i].extend(['[', ']'])
            case '@':
                wide_grid[i].extend(['@', '.'])
            case '.':
                wide_grid[i].extend(['.', '.'])

n, m = len(wide_grid), len(wide_grid[0])
rx, ry = [(x, y) for x in range(n) for y in range(m) if wide_grid[x][y] == '@'][0]

horiz = [(0, 1), (0, -1)]
visited = set()
def search(grid, x, y, dx, dy):
    global visited
    if (x, y) in visited:
        return False, []
    visited.add((x, y))
    to_move = []
    
    if x < 0 or x >= n or y < 0 or y >= m:
        return True, []
    if grid[x][y] == '#':
        return True, []
    if grid[x][y] == '.':
        return False, [(x, y)]
    
    stopped = False
    if (dx, dy) in horiz:
        s, mv = search(grid, x + dx, y + dy, dx, dy)
        if s:
            return True, []
        to_move.append((x, y))
        to_move.extend(mv)
    else:
        if grid[x][y] == '[':
            right_s, right_mv = search(grid, x, y + 1, dx, dy)
            if right_s:
                return True, []
            to_move.extend(right_mv)

        elif grid[x][y] == ']':
            left_s, left_mv = search(grid, x, y - 1, dx, dy)
            if left_s:
                return True, []
            to_move.extend(left_mv)

        d_s, d_mv = search(grid, x + dx, y, dx, dy)
        if d_s:
            return True, []
        
        to_move.append((x, y))
        to_move.extend(d_mv)

    match (dx, dy):
        case (0, 1): # greatest y goes first
            to_move.sort(key=lambda x: -x[1])
        case (0, -1): # smallest y goes first
            to_move.sort(key=lambda x: x[1])
        case (1, 0): # greatest x goes first
            to_move.sort(key=lambda x: -x[0])
        case (-1, 0): # smallest x goes first
            to_move.sort(key=lambda x: x[0])
    
    return stopped, to_move


def do_move(grid, sx, sy, dx, dy):
    fx, fy = sx + dx, sy + dy
    stopped, to_move = search(grid, fx, fy, dx, dy)
    # print(sx, sy, dx, dy, stopped, to_move)
    if stopped:
        return grid, sx, sy
    
    to_move.append((sx, sy))
    for x, y in to_move:
        if (x - dx, y - dy) in to_move:
            grid[x][y] = grid[x - dx][y - dy]
        else:
            grid[x][y] = '.'
    grid[sx][sy] = '.'

    return grid, sx + dx, sy + dy

for inst in insts:
    dx, dy = dirs[inst]
    visited = set()
    wide_grid, rx, ry = do_move(wide_grid, rx, ry, dx, dy)
    # print_grid(wide_grid)
    # input()

s = 0
for x in range(n):
    for y in range(m):
        if wide_grid[x][y] == '[':
            s += 100 * x + y

print(s)