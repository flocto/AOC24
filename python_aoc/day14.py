import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re
from tqdm import trange

day = 14
inp = open(f'../input/day{day}.txt').read().splitlines()

robots = []
for line in inp:
    p, v = line.split(' v=')
    p = p.split('p=')[1].split(',')
    p = tuple(map(int, p))
    v = tuple(map(int, v.split(',')))
    robots.append((p, v))

# part 1
n, m = 101, 103
grid = Counter()
for p, v in robots:
    px, py = p
    px += (v[0] * 100) % n
    py += (v[1] * 100) % m
    grid[(px % n, py % m)] += 1

# quadrants
quadrants = [
    (0, 0, n // 2, m // 2),
    (n // 2 + 1, 0, n, m // 2),
    (0, m // 2 + 1, n // 2, m),
    (n // 2 + 1, m // 2 + 1, n, m)
]

s = 1
for q in quadrants:
    print(q, sum(grid[(x, y)] for x in range(q[0], q[2]) for y in range(q[1], q[3])))
    s *= sum(grid[(x, y)] for x in range(q[0], q[2]) for y in range(q[1], q[3]))

print(s)

# part 2
def draw_grid(grid):
    for y in range(m):
        for x in range(n):
            if (x, y) in grid:
                print(grid[(x, y)], end='')
            else:
                print('.', end='')
        print()

for i in range(1, n * m):
    grid = Counter()
    new_robots = []
    for p, v in robots:
        px, py = p
        px = (px + v[0]) % n
        py = (py + v[1]) % m
        grid[(px, py)] += 1
        new_robots.append(((px, py), v))
    robots = new_robots

    if all(grid[(x, y)] == 1 for x, y in grid):
        draw_grid(grid)
        print(i)
        break