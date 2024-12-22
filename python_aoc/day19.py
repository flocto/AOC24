import math
from functools import cache

day = 19
types, wanteds = open(f'../input/day{day}.txt').read().split('\n\n')
types = list(set(types.split(', ')))
types.sort(key=lambda x: -len(x))
wanteds = wanteds.split('\n')

# part 1
@cache
def search(wanted):
    for pat in types:
        if wanted == pat:
            return True
        if wanted.startswith(pat):
            if search(wanted[len(pat):]):
                return True
    return False
    
print(sum(search(wanted) for wanted in wanteds))

# part 2
@cache
def search2(wanted):
    ways = 0
    for pat in types:
        if wanted == pat:
            ways += 1
        if wanted.startswith(pat):
            ways += search2(wanted[len(pat):])
    return ways
    
print(sum(search2(wanted) for wanted in wanteds))
