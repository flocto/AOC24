day = 17
inp = open(f'../input/day{day}.txt').read()
regs, inst = inp.split('\n\n')
regs = [r.split()[-1] for r in regs.split('\n')]
inst = list(map(int, inst.split()[1].split(',')))

# part 1
a, b, c = map(int, regs)
# a = 105706277661082 # part 2 verification xd
ip = 0
def combo(op):
    if 0 <= op <= 3:
        return op
    elif 4 <= op <= 6:
        return [a, b, c][op - 4]
    else:
        print('WTF')

def do_inst(op, arg):
    global a, b, c, ip
    # print(a, b, c, ip, op, arg)
    match op:
        case 0:
            a = a >> combo(arg)
        case 1:
            b = b ^ arg
        case 2:
            b = combo(arg) % 8
        case 3:
            if a != 0:
                ip = arg
                return
        case 4:
            b = b ^ c
        case 5:
            print(combo(arg) % 8, end=',', flush=True)
        case 6:
            b = a >> combo(arg)
        case 7:
            c = a >> combo(arg)

    ip += 2

while 0 <= ip < len(inst):
    op, arg = inst[ip:ip+2]
    do_inst(op, arg)

print()

# part 2

# b = a % 8
# b ^= 5
# c = a >> b
# b ^= 6
# a >>= 3
# b ^= c
# print b
# jmp to start while a

a_parts = []
prog = inst[::-1]

def dfs(prog, a, ct = 0):
    if ct == len(prog):
        return a
    
    prog_part = prog[ct]
    for i in range(8):
        a_ = (a << 3) + i
        b = a_ % 8
        b ^= 5
        c = a_ >> b
        b ^= 6
        # a_ >>= 3
        b ^= c
        b = b % 8
        if b == prog_part:
            res = dfs(prog, a_, ct + 1)
            if res:
                return res

import time
start = time.time()
a = dfs(prog, 0)
print(a, time.time() - start)

# import z3
# s = z3.Solver()
# a = orig_a = z3.BitVec('a', len(inst) * 3)

# for i, v in enumerate(inst):
#     b = a % 8
#     b ^= 5
#     c = z3.LShR(a, b)
#     b ^= 6
#     a = z3.LShR(a, 3)
#     b ^= c
#     s.add(b % 8 == v)

# best = 1 << (len(inst) * 3)
# while s.check() == z3.sat:
#     a_val = s.model()[orig_a].as_long()
#     if a_val < best:
#         best = a_val
#         print(best)
#     s.add(orig_a < a_val)
# else:
#     print('done')