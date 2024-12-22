from functools import cache
from collections import Counter, defaultdict
from tqdm import trange, tqdm

day = 22
inp = open(f'../input/day{day}.txt').read().splitlines()
secrets = [int(x) for x in inp]
N = len(secrets)

@cache  
def next(n):
    n ^= n << 6
    n %= 16777216
    n ^= n >> 5
    n ^= n << 11
    n %= 16777216
    return n

all_secrets = []

for n in tqdm(secrets):
    buyer = [n]
    for _ in range(2000):
        n = next(n)
        buyer.append(n)
    all_secrets.append(buyer)

# part 1
print(sum(buyer[-1] for buyer in all_secrets))

# part 2
bananas = Counter()

for i in trange(N):
    first_price = defaultdict(int)
    prices = [s % 10 for s in all_secrets[i]]
    price_diffs = [prices[j] - prices[j-1] for j in range(1, 2001)]
    last_4 = price_diffs[:4]
    for j in range(4, 2000):
        _last_4 = tuple(last_4)
        if _last_4 not in first_price:
            first_price[_last_4] = prices[j]
        last_4.pop(0)
        last_4.append(price_diffs[j])

    for monkey, banana in first_price.items():
        bananas[monkey] += banana

print(bananas.most_common(1)[0][1])
