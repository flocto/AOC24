dat = open('../input/day1.txt').read().splitlines()

# part 1
a, b = [], []
for row in dat:
    x, y = map(int, row.split())
    a.append(x)
    b.append(y)

a.sort()
b.sort()

d = 0
for x, y in zip(a, b):
    d += abs(x - y)
print(d)

# part 2
s = 0
for x in a:
    s += x * b.count(x)
print(s)
