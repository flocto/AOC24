import math
import itertools
from collections import Counter, defaultdict, deque
import bisect
import re

day = 5
dat = open(f'../input/day{day}.txt').read().split('\n\n')

# part 1 and 2
rules = dat[0].split('\n')
all_pages = dat[1].split('\n')
c1, c2 = 0, 0
rules = set(tuple(map(int, r.split('|'))) for r in rules)
all_pages = [list(map(int, p.split(','))) for p in all_pages]

for pages in all_pages:  
    seen = []
    broken = []
    impt = []
    for page in pages:
        for s in seen:
            if (page, s) in rules:
                broken.append((page, s))
            elif (s, page) in rules:
                impt.append((s, page))
        seen.append(page)
        
    if not broken:
        c1 += pages[len(pages) // 2]
        continue

    impt += broken
    fixed = True
    while fixed:
        fixed = False
        for (a, b) in impt:
            i, j = pages.index(a), pages.index(b)
            if i < j:
                continue
            pages[i], pages[j] = pages[j], pages[i]
            fixed = True

    c2 += pages[len(pages) // 2]

print(c1, c2)