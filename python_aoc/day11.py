import functools
from collections import Counter

day = 11
inp = open(f'../input/day{day}.txt').read()

nums = list(map(int, inp.split()))

def apply_rule(c, n, k):
    if n == 0:
        # return [1]
        # return Counter([1])
        c[1] += k
    
    elif len(str(n)) % 2 == 0:
        s = str(n)
        n = len(s) // 2
        # return [int(s[:n]), int(s[n:])]
        # return Counter([int(s[:n]), int(s[n:])])
        c[int(s[:n])] += k
        c[int(s[n:])] += k

    else:    
        # return [n * 2024]
        # return Counter([n * 2024])
        c[n * 2024] += k
    
    return c

@functools.lru_cache(maxsize=None)
def blink(n, k):
    stones = Counter([n])
    for _ in range(k):
        new_stones = Counter()
        for s in stones:
            apply_rule(new_stones, s, stones[s])
        stones = new_stones
    return stones

@functools.lru_cache(maxsize=None)
def rec(n, d=0, k=15):
    next_stage = blink(n, 5)
    if d == k - 1:
        return sum(next_stage.values())

    return sum(rec(s, d+1, k) * next_stage[s] for s in next_stage)

# part 1
print(sum(rec(n, 0, 5) for n in nums))

# part 2
print(sum(rec(n) for n in nums))