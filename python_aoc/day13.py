import re
import z3

day = 13
inp = open(f'../input/day{day}.txt').read().split('\n\n')
format = r'Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)'
machines = [
    list(map(int, re.findall(format, part)[0]))
    for part in inp
]

s = 0
for (x1, y1, x2, y2, px, py) in machines:
    sol = z3.Solver()
    a, b = z3.Ints('a b')

    # sol.add(a * x1 + b * x2 == px)
    # sol.add(a * y1 + b * y2 == py)
    sol.add(a * x1 + b * x2 == px + 10000000000000)
    sol.add(a * y1 + b * y2 == py + 10000000000000)

    if sol.check() == z3.sat:
        m = sol.model()
        s += m[a].as_long() * 3 + m[b].as_long()
    else:
        # print('unsat')
        pass

print(s)